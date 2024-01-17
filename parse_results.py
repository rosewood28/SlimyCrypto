import base64
import json
 
# Opening JSON file
f = open('result.json')
 
# returns JSON object as 
# a dictionary
data = json.load(f)
 
# Iterating through the json
# list

if "results" in data:
    for i in data['results']:
        tx_data = i['data']
        output = base64.b64decode(tx_data).decode("ASCII")
        print(output)
 
# Closing file
f.close()