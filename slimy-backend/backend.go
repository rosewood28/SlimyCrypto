package main

import (
	"encoding/base64"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"github.com/emicklei/go-restful/v3"
	"io"
	"log"
	"net/http"
	"os/exec"
	"slices"
	"strconv"
	"strings"
)

const API = "https://testnet-api.multiversx.com"
const devScript = "dev-interact.sh"
const devBreedFunc = "dev_breed"

type FightStatus uint8
type WildSlimeStatus uint8
type Move uint8

const (
	WaitingOnPlayer FightStatus = iota
	WaitingOnTx
	PlayerLost
	PlayerWon
)

const (
	Fighting WildSlimeStatus = iota
	CanCatch
)

const (
	Attack Move = iota
	Buff
	Heal
	Catch
)

type SlimeFight struct {
	Attack uint8
	Def    uint8
	MaxHP  uint8
	CurrHP uint8
}

type FightMove struct {
	Hunter     string `json:"hunter"`
	MovePicked Move   `json:"move"`
}

type FightInit struct {
	Hunter  string `json:"hunter"`
	SlimeID uint64 `json:"slime_id"`
}

type Fight struct {
	Wild       SlimeFight      `json:"wild"`
	Attacker   SlimeFight      `json:"attacker"`
	Hunter     string          `json:"hunter"`
	Status     FightStatus     `json:"status"`
	WildStatus WildSlimeStatus `json:"wild_status"`
}

type Slime struct {
	SlimeID uint64 `json:"slime_id"`
}

type Database struct {
	Fights          map[uint]Fight
	Users           []string
	SeenTransaction []string
}

type TXResult struct {
	Hash           string `json:"hash"`
	Timestamp      int    `json:"timestamp"`
	Nonce          int    `json:"nonce"`
	GasLimit       int    `json:"gasLimit"`
	GasPrice       int    `json:"gasPrice"`
	Value          string `json:"value"`
	Sender         string `json:"sender"`
	Receiver       string `json:"receiver"`
	Data           string `json:"data"`
	PrevTxHash     string `json:"prevTxHash"`
	OriginalTxHash string `json:"originalTxHash"`
	CallType       string `json:"callType"`
	MiniBlockHash  string `json:"miniBlockHash"`
	Function       string `json:"function"`
}

type APIResult struct {
	Results []TXResult `json:"results"`
}

type APIData struct {
	Data string `json:"data"`
}

var db Database

func main() {
	db = Database{
		Fights:          make(map[uint]Fight),
		Users:           make([]string, 0),
		SeenTransaction: make([]string, 0),
	}

	ws := new(restful.WebService)

	ws.Route(ws.GET("/register_user/{tx}").
		Produces(restful.MIME_JSON).
		Doc("Registers new user").
		To(handleRegister))

	ws.Route(ws.GET("/item/{tx}").
		Produces(restful.MIME_JSON).
		Doc("Acknowledges item use.").
		To(handleItem))

	ws.Route(ws.GET("/catch/{tx}").
		Produces(restful.MIME_JSON).
		Doc("Acknowledges catch").
		To(handleCatch))

	ws.Route(ws.GET("/breed/{tx}").
		Consumes(restful.MIME_JSON).
		Doc("Breeds slimes").
		Returns(200, "OK", Slime{}).
		Returns(500, "Internal Server Error", nil).
		To(handleBreed))

	ws.Route(ws.GET("/gen/{tx}").
		Produces(restful.MIME_JSON).
		Doc("Generates wild slimes").
		Returns(200, "OK", Slime{}).
		Returns(500, "Internal Server Error", nil).
		To(handleGen))

	ws.Route(ws.GET("/fight/{wild_slime_id}").
		Produces(restful.MIME_JSON).
		Doc("Starts a fight").
		Reads(FightInit{}).
		Writes(Fight{}). // on the response
		Returns(200, "OK", Fight{}).
		Returns(500, "Internal Server Error", nil).
		To(handleFightStart))

	ws.Route(ws.GET("/fight_move/{wild_slime_id}").
		Produces(restful.MIME_JSON).
		Reads(FightMove{}).
		Writes(Fight{}).
		Doc("Sends a fight move").
		Returns(200, "OK", Fight{}).
		Returns(500, "Internal Server Error", nil).
		To(handleFightMoves))

	cors := restful.CrossOriginResourceSharing{
		AllowedHeaders: []string{"Content-Type", "Accept"},
		AllowedDomains: []string{},
		AllowedMethods: []string{"POST"},
		CookiesAllowed: true,
		Container:      restful.DefaultContainer}
	restful.DefaultContainer.Filter(cors.Filter)
	restful.Add(ws)

	log.Fatal(http.ListenAndServe(":8080", nil))
}

func handleFightMoves(request *restful.Request, response *restful.Response) {

}

func handleFightStart(request *restful.Request, response *restful.Response) {

}

func handleGen(request *restful.Request, response *restful.Response) {

}

func handleItem(request *restful.Request, response *restful.Response) {

}

func handleCatch(request *restful.Request, response *restful.Response) {

}

func apiRoute(tx string, fields string) string {
	return API + "/transactions/" + tx + "?fields=" + fields
}

func handleBreed(req *restful.Request, resp *restful.Response) {
	tx := req.PathParameter("tx")

	if slices.Contains(db.SeenTransaction, tx) {
		resp.WriteHeader(http.StatusAlreadyReported)
		return
	}

	db.SeenTransaction = append(db.SeenTransaction, tx)

	res, err := http.Get(apiRoute(tx, "data"))
	if err != nil {
		_ = resp.WriteError(http.StatusInternalServerError, err)
		return
	}

	responseData, err := io.ReadAll(res.Body)
	if err != nil {
		log.Println(err)
		_ = resp.WriteError(http.StatusInternalServerError, err)
		return
	}

	var result APIData
	err = json.Unmarshal(responseData, &result)
	if err != nil {
		log.Println(err)
		resp.WriteHeader(http.StatusInternalServerError)
		return
	}

	correct, err := checkTxTransfer(result.Data, 1, "534c4b2d376435616633", 2, 20)
	if err != nil {
		log.Printf("error checkTxTransfer: %v", err)
		resp.WriteHeader(http.StatusInternalServerError)
		return
	}

	if !correct {
		_ = resp.WriteError(http.StatusBadRequest, errors.New("expected 20SLK or more transfer"))
		return
	}

	correct, out, err := checkTxData(result.Data, 3, "breed", 3)
	if err != nil {
		log.Printf("error checkTxData: %v", err)
		resp.WriteHeader(http.StatusInternalServerError)
		return
	}

	if !correct {
		log.Printf("Wrong data for breed: %s", out)
		_ = resp.WriteError(http.StatusBadRequest, errors.New(fmt.Sprintf("expected 'breed {id_1} {id_2}' as data on transfer, received %s", out)))
		return
	}

	stringSplit := strings.Split(out, " ")
	cmd := exec.Command("bash", "-c", fmt.Sprintf(". %s && %s %s %s", devScript, devBreedFunc, stringSplit[1], stringSplit[2]))
	stdout, err := cmd.CombinedOutput()

	if err != nil {
		log.Printf("error on exec %v", err)
		log.Println(string(stdout))
		resp.WriteHeader(http.StatusInternalServerError)
		return
	}

	_ = resp.WriteError(http.StatusOK, errors.New(string(stdout)))
}

func handleRegister(req *restful.Request, resp *restful.Response) {
	tx := req.PathParameter("tx")

	if slices.Contains(db.SeenTransaction, tx) {
		resp.WriteHeader(http.StatusAlreadyReported)
		return
	}

	db.SeenTransaction = append(db.SeenTransaction, tx)

	res, err := http.Get(apiRoute(tx, "results"))
	if err != nil {
		_ = resp.WriteError(http.StatusInternalServerError, err)
		return
	}

	responseData, err := io.ReadAll(res.Body)
	if err != nil {
		log.Println(err)
		_ = resp.WriteError(http.StatusInternalServerError, err)
		return
	}

	var result APIResult
	err = json.Unmarshal(responseData, &result)
	if err != nil {
		log.Println(err)
		resp.WriteHeader(http.StatusInternalServerError)
		return
	}

	for _, txRes := range result.Results {
		correct, err := checkTxRes(txRes, "ESDTTransfer", 1, "534c4b2d376435616633")
		if err != nil {
			log.Println(err)
			resp.WriteHeader(http.StatusInternalServerError)
			return
		}

		if correct {
			if slices.Contains(db.Users, txRes.Receiver) {
				_ = resp.WriteError(http.StatusBadRequest, errors.New("user already registered"))
				return
			}
			db.Users = append(db.Users, txRes.Receiver)

			resp.WriteHeader(http.StatusOK)
			return
		}
	}

	_ = resp.WriteError(http.StatusBadRequest, errors.New("transaction does not acquire SLK"))
	return
}

func checkTxRes(txRes TXResult, function string, split int, split_val string) (bool, error) {
	if txRes.Function == function {
		base64data, err := base64.StdEncoding.DecodeString(txRes.Data)
		if err != nil {
			return false, err
		}
		transferString := string(base64data)
		stringSplit := strings.Split(transferString, "@")
		if stringSplit[split] == split_val {
			return true, nil
		}
	}
	return false, nil
}

func checkTxTransfer(data string, tokLoc int, expectedTok string, valLoc int, expectedVal int) (bool, error) {
	base64data, err := base64.StdEncoding.DecodeString(data)
	if err != nil {
		return false, err
	}

	transferString := string(base64data)
	stringSplit := strings.Split(transferString, "@")

	if stringSplit[tokLoc] != expectedTok {
		return false, nil
	}

	transferVal, err := strconv.ParseInt(stringSplit[valLoc], 0, 64)
	if err != nil {
		return false, err
	}

	if transferVal >= int64(expectedVal) {
		return true, nil
	}

	return false, nil
}

func checkTxData(data string, dataSplitLoc int, expectedFirst string, expectedNumArgs int) (bool, string, error) {
	base64data, err := base64.StdEncoding.DecodeString(data)
	if err != nil {
		return false, "", err
	}

	transferString := string(base64data)
	stringSplit := strings.Split(transferString, "@")

	base64data, err = hex.DecodeString(stringSplit[dataSplitLoc])
	if err != nil {
		return false, "", err
	}

	transferString = string(base64data)
	stringSplit = strings.Split(transferString, " ")
	if stringSplit[0] == expectedFirst || len(stringSplit) != expectedNumArgs {
		return true, transferString, nil
	}

	return false, transferString, nil
}
