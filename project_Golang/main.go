package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/rand"
	"net/http"
	"time"
)

type Payload struct {
	GoroutineID string `json:"goroutine_id"`
	Timestamp   string `json:"timestamp"`
}

func main() {
	http.HandleFunc("/create-goroutines", createGoroutines)
	fmt.Println("Starting Go server on :8081")
	http.ListenAndServe(":8081", nil)
}

func createGoroutines(w http.ResponseWriter, r *http.Request) {
	numGoroutines := rand.Intn(10) + 1 // Create a random number of Goroutines (1-10)
	fmt.Printf("Creating %d goroutines\n", numGoroutines)

	for i := 0; i < numGoroutines; i++ {
		go func(id int) {
			goroutineID := fmt.Sprintf("G%d", id)
			endTime := time.Now().Add(15 * time.Second) // Goroutine will run for less than 15 seconds
			for time.Now().Before(endTime) {
				payload := Payload{
					GoroutineID: goroutineID,
					Timestamp:   time.Now().UTC().Format(time.RFC3339),
				}
				fmt.Printf("Sending payload: %+v\n", payload)
				sendPayload(payload)
				time.Sleep(2 * time.Second) // Send data every 2 seconds
			}
		}(i)
	}

	fmt.Fprintf(w, "Created %d goroutines\n", numGoroutines)
}

func sendPayload(payload Payload) {
	jsonData, err := json.Marshal(payload)
	if err != nil {
		fmt.Println("Error marshaling JSON:", err)
		return
	}

	resp, err := http.Post("http://127.0.0.1:8000/receive", "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		fmt.Println("Error sending payload:", err)
		return
	}
	defer resp.Body.Close()

	if resp.StatusCode == http.StatusOK {
		fmt.Println("Payload sent successfully")
	} else {
		fmt.Println("Failed to send payload")
	}
}

