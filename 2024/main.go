package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/go-chi/chi/v5"
	"github.com/go-chi/chi/v5/middleware"
	_01 "github.com/rathalos64/adventofcode/2024/_01"
	_02 "github.com/rathalos64/adventofcode/2024/_02"
	"github.com/rathalos64/adventofcode/2024/_03"
	"github.com/rathalos64/adventofcode/2024/core"
)

type Controller struct {
	Exercises map[string]core.Exercise
}

func main() {
	controller := Controller{
		Exercises: map[string]core.Exercise{},
	}
	err := controller.RegisterExercises()
	if err != nil {
		log.Fatalf("failed to register exercises: %s", err)
	}

	router := chi.NewRouter()
	router.Use(middleware.Logger)
	router.Use(middleware.Recoverer)
	router.Get("/advent-of-code/2024/{exercise}", controller.ExerciseR)
	err = http.ListenAndServe(":2024", router)
	log.Fatalf("oh no.. %s", err)
}

func (controller *Controller) RegisterExercises() error {
	e1, _, err := _01.GetExercise()
	if err != nil {
		return fmt.Errorf("failed to register exercise 01: %w", err)
	}
	e2, _, err := _02.GetExercise()
	if err != nil {
		return fmt.Errorf("failed to register exercise 02: %w", err)
	}
	e3, _, err := _03.GetExercise()
	if err != nil {
		return fmt.Errorf("failed to register exercise 02: %w", err)
	}
	controller.Exercises["01"] = e1
	controller.Exercises["02"] = e2
	controller.Exercises["03"] = e3
	return nil
}

func (controller *Controller) ExerciseR(w http.ResponseWriter, r *http.Request) {
	number := chi.URLParam(r, "exercise")
	exercise, ok := controller.Exercises[number]
	if !ok {
		w.WriteHeader(http.StatusNotFound)
		w.Write([]byte("could not find this execise, maybe it's not yet implemented"))
		return
	}

	res1, duration1, err := exercise.Task1()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte(fmt.Sprintf("oh not something happened with task1: %s", err)))
		return
	}
	res2, duration2, err := exercise.Task2()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		w.Write([]byte(fmt.Sprintf("oh not something happened with task2: %s", err)))
		return
	}

	w.WriteHeader(http.StatusOK)
	w.Write([]byte(fmt.Sprintf("> task 1 took %s\n", duration1.String())))
	w.Write([]byte(fmt.Sprintf("result %d", res1)))
	w.Write([]byte("\n\n"))

	w.Write([]byte(fmt.Sprintf("> task 2 took %s\n", duration2.String())))
	w.Write([]byte(fmt.Sprintf("result %d", res2)))
	w.Write([]byte("\n\n"))

	w.Write([]byte("==========================================================="))
	w.Write([]byte("\n\n"))
	w.Write([]byte(exercise.GetDescription()))
	w.Write([]byte("\n\n"))

	w.Write([]byte("==========================================================="))
	w.Write([]byte("\n\n"))
	w.Write([]byte(exercise.GetInput()))
}
