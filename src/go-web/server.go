package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
)

func handleFileRoute(rw http.ResponseWriter, r *http.Request) {
	requestedFilepath := r.URL.Path[1:]
	log.Printf("path /%s requested\n", requestedFilepath)
	http.ServeFile(rw, r, "./static/"+requestedFilepath)
}

func handleSheetsRoute(rw http.ResponseWriter, r *http.Request) {
	log.Printf("path /sheets/ requested\n")
	fmt.Fprintf(rw, "path /sheets/ requested\n")
}

func main() {
	log.SetOutput(os.Stdout)

	// assign handler functions to filepaths
	http.HandleFunc("/", handleFileRoute)
	http.HandleFunc("/sheets/", handleSheetsRoute)

	// spin up a server listening for HTTP traffic on port 8081
	log.Fatal(http.ListenAndServe(":8081", nil))
}
