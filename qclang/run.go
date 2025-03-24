package qclang

import (
	"encoding/json"
	"log"
)

func formatOutputs(outputs map[string]uint) string {
	b, err := json.MarshalIndent(outputs, "", "")
	if err != nil {
		log.Fatalln("[ERROR]", err)
	}
	return string(b[2 : len(b)-2])
}
