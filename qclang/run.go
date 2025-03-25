package qclang

import (
	"encoding/json"
	"log"
	"strings"
)

func formatOutputs(outputs map[string]uint) string {
	b, err := json.MarshalIndent(outputs, "", "")
	if err != nil {
		log.Fatalln("[ERROR]", err)
	}
	output := strings.ReplaceAll(string(b), ",", "")
	return strings.Trim(output, "{\n}")
}
