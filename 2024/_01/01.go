package _01

import (
	"embed"
	"fmt"
	"math"
	"sort"
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

	Group1 []int
	Group2 []int
}

func (e *exercise) processInput() error {
	lines := strings.Split(e.Input, "\n")
	for _, line := range lines {
		groups := strings.Fields(strings.TrimSpace(line))
		if len(groups) != 2 {
			return fmt.Errorf("invalid number of groups, expected 2, got %d (in line %q)", len(groups), line)
		}

		g1, err := strconv.Atoi(groups[0])
		if err != nil {
			return fmt.Errorf("expected number, got whatever (%s): %w", groups[0], err)
		}
		g2, err := strconv.Atoi(groups[1])
		if err != nil {
			return fmt.Errorf("expected number, got whatever (%s): %w", groups[1], err)
		}

		e.Group1 = append(e.Group1, g1)
		e.Group2 = append(e.Group2, g2)
	}

	return nil
}

func (e *exercise) Task1() (int, time.Duration, error) {
	start := time.Now()

	sort.Ints(e.Group1)
	sort.Ints(e.Group2)

	sumOfDistance := 0.0
	for i := range len(e.Group1) {
		sumOfDistance += math.Abs(float64(e.Group1[i] - e.Group2[i]))
	}
	return int(sumOfDistance), time.Since(start), nil
}

func (e *exercise) Task2() (int, time.Duration, error) {
	start := time.Now()

	occurrences := map[int]int{}
	for _, locationID := range e.Group2 { // right
		occurrences[locationID]++
	}

	similarityScoreTotal := 0
	for _, locationID := range e.Group1 { // left
		similarityScoreTotal += (locationID * occurrences[locationID])
	}

	return similarityScoreTotal, time.Since(start), nil
}

func (e *exercise) GetInput() string {
	return e.Input
}

func (e *exercise) GetDescription() string {
	return description
}
