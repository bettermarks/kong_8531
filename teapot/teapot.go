package main

import (
	"log"
	"net/http"
)

func main() {

	log.Println("Listening on :8080")
	http.ListenAndServe(":8080", http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(418)
	}))
}
