package _04

import (
	"embed"
	"fmt"
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

	WordSearch []string
	Length     int   // length per line
	XIndices   []int // where do X's start
	AIndices   []int // where do A's start
}

const (
	Xmas = "XMAS"
	Mas  = "MAS"
	Sam  = "SAM"
)

func (e *exercise) processInput() error {
	lines := strings.Split(e.Input, "\n")
	e.Length = len(lines[0])
	for i, line := range lines {
		if len(line) != e.Length {
			return fmt.Errorf("expected length (%d), got (%d) in %dth line %s", e.Length, len(line), i, line)
		}
		for x, l, d := strings.Index(line, "X"), line, 0; x > -1; x, d = strings.Index(l, "X"), d+x+1 {
			e.XIndices = append(e.XIndices, (i*e.Length)+(x+d))
			l = l[x+1:]
		}
		for x, l, d := strings.Index(line, "A"), line, 0; x > -1; x, d = strings.Index(l, "A"), d+x+1 {
			e.AIndices = append(e.AIndices, (i*e.Length)+(x+d))
			l = l[x+1:]
		}

		e.WordSearch = append(e.WordSearch, strings.Split(line, "")...)
	}

	return nil
}

func (e *exercise) Task1() (int, time.Duration, error) {
	start := time.Now()

	// assume x1 is always the X
	getWord := func(horizontal, backward bool, x1, x2, x3, x4 int) string {
		if x1 < 0 || x2 < 0 || x3 < 0 || x4 < 0 {
			return ""
		}
		if x1 >= len(e.WordSearch) || x2 >= len(e.WordSearch) ||
			x3 >= len(e.WordSearch) || x4 >= len(e.WordSearch) {
			return ""
		}
		if horizontal {
			if backward {
				if (x1 % e.Length) < 3 { // left padding
					return ""
				}
			} else {
				if e.Length-(x1%e.Length) < 4 { // right padding
					return ""
				}
			}
		}
		return e.WordSearch[x1] + e.WordSearch[x2] + e.WordSearch[x3] + e.WordSearch[x4]
	}

	isXmas := func(s string) int {
		if s == Xmas {
			return 1
		}
		return 0
	}

	ocs := 0
	for _, x := range e.XIndices {
		// clockwise in 8ths
		ocs += isXmas(getWord(false, true, x, x-(e.Length), x-(2*e.Length), x-(3*e.Length)))
		ocs += isXmas(getWord(true, false, x, (x+1)-(e.Length), (x+2)-(2*e.Length), (x+3)-(3*e.Length)))
		ocs += isXmas(getWord(true, false, x, x+1, x+2, x+3))
		ocs += isXmas(getWord(true, false, x, (x+1)+(e.Length), (x+2)+(2*e.Length), (x+3)+(3*e.Length)))
		ocs += isXmas(getWord(false, false, x, x+(e.Length), x+(2*e.Length), x+(3*e.Length)))
		ocs += isXmas(getWord(true, true, x, (x-1)+(e.Length), (x-2)+(2*e.Length), (x-3)+(3*e.Length)))
		ocs += isXmas(getWord(true, true, x, x-1, x-2, x-3))
		ocs += isXmas(getWord(true, true, x, (x-1)-(e.Length), (x-2)-(2*e.Length), (x-3)-(3*e.Length)))
	}

	return ocs, time.Since(start), nil
}

func (e *exercise) Task2() (int, time.Duration, error) {
	start := time.Now()

	isXmas := func(x int) bool {
		if x%e.Length == 0 || x%e.Length == e.Length-1 { // left/right bounds
			return false
		}
		if x-e.Length < 0 || x+e.Length >= len(e.WordSearch) { // top/bottom bounds
			return false
		}

		firstDiag := e.WordSearch[(x-1)-(e.Length)] + e.WordSearch[x] + e.WordSearch[(x+1)+(e.Length)]
		secondDiag := e.WordSearch[(x-1)+(e.Length)] + e.WordSearch[x] + e.WordSearch[(x+1)-(e.Length)]
		return (firstDiag == Mas || firstDiag == Sam) && (secondDiag == Mas || secondDiag == Sam)
	}

	ocs := 0
	for _, x := range e.AIndices {
		if isXmas(x) {
			ocs++
		}
	}

	return ocs, time.Since(start), nil
}

func (e *exercise) GetInput() string {
	return e.Input
}

func (e *exercise) GetDescription() string {
	return description
}
