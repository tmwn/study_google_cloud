package main

import (
	"fmt"
	"html"
	"log"
	"net/http"
	"os"
)

func main() {
	secret := os.Getenv("SECRET")
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Hello, secret key %s!", html.EscapeString(secret))
	})
	log.Fatal(http.ListenAndServe("0.0.0.0:8080", nil))
}
