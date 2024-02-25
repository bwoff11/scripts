package main

import (
	"fmt"
	"os"
	"path/filepath"
	"strings"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
)

func main() {
	// Initialize zerolog with console writer for better readability
	log.Logger = log.Output(zerolog.ConsoleWriter{Out: os.Stderr})

	// Check if the correct number of arguments are passed
	if len(os.Args) != 2 {
		log.Error().Msg("Incorrect usage. Please specify a directory path.")
		fmt.Println("Usage: go run main.go <path_to_directory>")
		return
	}

	// Get the root directory from the command line arguments
	rootDir := os.Args[1]
	log.Info().Msgf("Starting to search in: %s", rootDir)

	// Walk through the root directory and its subdirectories
	err := filepath.Walk(rootDir, func(path string, info os.FileInfo, err error) error {
		if err != nil {
			// Log any errors encountered during filepath.Walk
			log.Error().Err(err).Msg("Error encountered during directory traversal")
			return err
		}

		// Only handle directories (excluding the root directory itself)
		if info.IsDir() && path != rootDir {
			log.Debug().Msgf("Inspecting directory: %s", path)
			return handleDirectory(path)
		}

		return nil
	})

	if err != nil {
		log.Error().Err(err).Msg("Error walking through directories")
	} else {
		log.Info().Msg("Directory cleanup completed successfully.")
	}
}

func handleDirectory(dirPath string) error {
	// Read the contents of the directory
	log.Debug().Msgf("Reading directory: %s", dirPath)
	files, err := os.ReadDir(dirPath)
	if err != nil {
		log.Error().Err(err).Msgf("Failed to read directory: %s", dirPath)
		return err
	}

	// Check if the directory contains exactly one file and it is an image file
	if len(files) == 1 && isImageFile(files[0].Name()) {
		log.Info().Msgf("Found a directory with a single image file: %s", dirPath)

		// Attempt to remove the directory
		err = os.RemoveAll(dirPath)
		if err != nil {
			log.Error().Err(err).Msgf("Failed to delete directory: %s", dirPath)
			return err
		}
		log.Info().Msgf("Successfully deleted: %s", dirPath)
	}

	// Skip this directory in the Walk function
	return filepath.SkipDir
}

func isImageFile(fileName string) bool {
	// Define a list of image file extensions
	imageExtensions := []string{".jpg", ".jpeg", ".png", ".gif", ".bmp"}

	// Check if the file has one of the defined image extensions
	for _, ext := range imageExtensions {
		if strings.HasSuffix(strings.ToLower(fileName), ext) {
			return true
		}
	}
	return false
}
