package main

import (
	"crypto/sha256"
	"fmt"
	"io"
	"log"
	"os"
	"path/filepath"
)

func main() {
	h := sha256.New()
	for _, arg := range os.Args[1:] {
		if err := filepath.Walk(arg, func(p string, fi os.FileInfo, err error) error {
			if !fi.Mode().IsRegular() {
				return nil
			}

			f, err := os.Open(p)
			if err != nil {
				return err
			}
			defer f.Close()

			h.Reset()
			if _, err := io.Copy(h, f); err != nil {
				return err
			}

			fmt.Printf("%x\t%s\n", h.Sum(nil), p)

			return nil
		}); err != nil {
			log.Fatalln(err)
		}
	}
}
