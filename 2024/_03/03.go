package _03

import (
	"embed"
	"fmt"
	"strconv"
	"strings"
	"time"

	"github.com/rathalos64/adventofcode/2024/core"
)

//go:embed "input"
var input embed.FS

func GetExercise() (core.Exercise, time.Duration, error) {
	start := time.Now()

	e := exercise{}
	content, err := input.ReadFile("input")
	if err != nil {
		return nil, 0, fmt.Errorf("failed to read input file: %w", err)
	}
	e.Input = string(content)

	err = e.processInput()
	if err != nil {
		return nil, 0, fmt.Errorf("failed to process input: %w", err)
	}

	return &e, time.Since(start), nil
}

type exercise struct {
	Input       string
	Description string

	Multiplications []string
}

const (
	multiplicaton = "mul"
	enable        = "do()"
	disable       = "don't()"
)

func (e *exercise) processInput() error {
	lines := strings.Split(e.Input, "\n")
	e.Multiplications = strings.Split(strings.Join(lines, ""), multiplicaton)
	if len(e.Multiplications) == 0 {
		return fmt.Errorf("expected to have any multiplication, got zero (%q)", e.Input)
	}

	return nil
}

func (e *exercise) Task1() (int, time.Duration, error) {
	start := time.Now()

	// filter out correct multiplications
	sum := 0
	for _, mults := range e.Multiplications {
		res := parseMultiplication(mults)
		if res != -1 {
			sum += res
		}
	}

	return sum, time.Since(start), nil
}

func (e *exercise) Task2() (int, time.Duration, error) {
	start := time.Now()

	// filter out correct multiplications
	sum := 0
	enabled := true
	for _, mults := range e.Multiplications {
		res := parseMultiplication(mults)
		if res != -1 && enabled {
			sum += res
		}

		enableI := strings.LastIndex(mults, enable)
		disableI := strings.LastIndex(mults, disable)
		if enableI != -1 && enableI > disableI {
			enabled = true
		}
		if disableI != -1 && disableI > enableI {
			enabled = false
		}
	}

	return sum, time.Since(start), nil
}

// if -1, it means error
func parseMultiplication(mults string) int {
	if len(mults) < 5 { // (1-3digit,1-3digit)
		return -1
	}
	if len(mults) > 9 {
		mults = mults[:9]
	}
	closingEnd := strings.Index(mults, ")")
	if mults[0] != '(' || closingEnd == -1 {
		return -1
	}
	mults = mults[1:closingEnd]
	comma := strings.Index(mults, ",")
	if comma == -1 {
		return -1
	}

	leftS := mults[:comma]
	rightS := mults[comma+1:]

	left, err := strconv.Atoi(leftS)
	if err != nil {
		return -1
	}
	right, err := strconv.Atoi(rightS)
	if err != nil {
		return -1
	}
	return left * right
}

func (e *exercise) GetInput() string {
	return e.Input
}

func (e *exercise) GetDescription() string {
	return description
}
