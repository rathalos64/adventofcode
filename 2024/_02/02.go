package _02

import (
	"embed"
	"fmt"
	"math"
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

	Reports [][]int
}

func (e *exercise) processInput() error {
	lines := strings.Split(e.Input, "\n")
	for _, line := range lines {
		report := strings.Fields(strings.TrimSpace(line))
		if len(report) == 0 {
			return fmt.Errorf("report has no levels (in line %q)", line)
		}

		levels := []int{}
		for _, slevel := range report {
			level, err := strconv.Atoi(slevel)
			if err != nil {
				return fmt.Errorf("expected number, got whatever (%s): %w", slevel, err)
			}
			levels = append(levels, level)
		}

		e.Reports = append(e.Reports, levels)
	}

	return nil
}

func (e *exercise) Task1() (int, time.Duration, error) {
	start := time.Now()

	safeReports := 0
	for _, report := range e.Reports {
		isSafe := true
		isIncreasing := true
		if report[0] > report[1] {
			isIncreasing = false
		}

		for i := range len(report) - 1 {
			if isIncreasing && report[i+1] < report[i] {
				isSafe = false
				break
			}
			if !isIncreasing && report[i+1] > report[i] {
				isSafe = false
				break
			}
			diff := int(math.Abs(float64(report[i+1] - report[i])))
			if !(1 <= diff && diff <= 3) {
				isSafe = false
				break
			}
		}

		if isSafe {
			safeReports++
		}
	}

	return safeReports, time.Since(start), nil
}

func (e *exercise) Task2() (int, time.Duration, error) {
	start := time.Now()

	safeReports := 0
	dampenerLimit := 1
	for _, report := range e.Reports {
		levelErrors := 0
		isIncreasing := true
		if report[0] > report[1] {
			isIncreasing = false
		}

		for i := range len(report) - 1 {
			if isIncreasing && report[i+1] < report[i] {
				levelErrors++
				continue
			}
			if !isIncreasing && report[i+1] > report[i] {
				levelErrors++
				continue
			}
			diff := int(math.Abs(float64(report[i+1] - report[i])))
			if !(1 <= diff && diff <= 3) {
				levelErrors++
				continue
			}
		}

		if levelErrors <= dampenerLimit {
			safeReports++
		}
	}

	return safeReports, time.Since(start), nil
}

func (e *exercise) GetInput() string {
	return e.Input
}

func (e *exercise) GetDescription() string {
	return description
}
