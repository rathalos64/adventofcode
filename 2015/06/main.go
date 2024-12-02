package main

import (
	"flag"
	"fmt"
	"io/ioutil"
	"log"
	"math"
	"strconv"
	"strings"
)

type Point struct {
	X int
	Y int
}

type Action string

const (
	ActionTurnOn  Action = "turn on"
	ActionTurnOff Action = "turn off"
	ActionToggle  Action = "toggle"
)

var Actions = []Action{
	ActionTurnOn,
	ActionTurnOff,
	ActionToggle,
}

type Operation struct {
	From   Point
	To     Point
	Action Action
}

func (o Operation) IsInRange(p Point) bool {
	return p.X >= o.From.X && p.Y >= o.From.Y && p.X <= o.To.X && p.Y <= o.To.Y
}

func (o Operation) ApplySwitch(state bool) bool {
	switch o.Action {
	case ActionTurnOff:
		return false
	case ActionTurnOn:
		return true
	case ActionToggle:
		return !state
	}

	log.Fatalf("should not reach here, invalid operation: %s", o.Action)
	return false
}

func (o Operation) ApplyBrightness(brightness int) int {
	switch o.Action {
	case ActionTurnOff:
		return int(math.Max(float64(brightness)-1, 0))
	case ActionTurnOn:
		return brightness + 1
	case ActionToggle:
		return brightness + 2
	}

	log.Fatalf("should not reach here, invalid operation: %s", o.Action)
	return -1
}

func main() {
	input := flag.String("input", "input", "the name of the input file")
	flag.Parse()

	content, err := ioutil.ReadFile(*input)
	if err != nil {
		log.Fatalf("failed to read %s: %s", *input, err)
	}

	// parse and put into structure
	operations, err := parseOperations(string(content))
	if err != nil {
		log.Fatalf("failed to parse operations: %s", err)
	}

	log.Printf("got %d operations", len(operations))

	// start apply operations on the grid
	lit := 0             // part one, count lit lights
	totalBrightness := 0 // part two, total brightness

	for x := range 1000 {
		for y := range 1000 {
			p := Point{x, y}
			lightState := false // false is not lit
			brightness := 0

			for _, operation := range operations {
				if operation.IsInRange(p) {
					lightState = operation.ApplySwitch(lightState)     // part one
					brightness = operation.ApplyBrightness(brightness) // part two
				}
			}

			// part one
			if lightState {
				lit++
			}

			// part two
			totalBrightness += brightness
		}
	}

	log.Println("after evaluating all operations on a 1000 x 1000 lights grid, we get")
	log.Printf("[part one] %d lit lights", lit)
	log.Printf("[part two] a total brightness of %d", totalBrightness)
}

func parseOperations(content string) ([]Operation, error) {
	operations := []Operation{}
	for i, line := range strings.Split(content, "\n") {
		line = strings.TrimSpace(line)
		if line == "" {
			continue
		}

		action, after, err := determineAction(line)
		if err != nil {
			return nil, fmt.Errorf("in line %d | line has invalid action %q", i, line)
		}

		parts := strings.Fields(after)
		if len(parts) != 3 {
			return nil, fmt.Errorf("in line %d | expected 3 parts, got %d", i, len(parts))
		}

		fromParts := strings.Split(parts[0], ",")
		toParts := strings.Split(parts[2], ",")

		fromX, err := strconv.Atoi(fromParts[0])
		if err != nil {
			return nil, fmt.Errorf("in line %d | failed to part from X: %s", i, err)
		}
		fromY, err := strconv.Atoi(fromParts[1])
		if err != nil {
			return nil, fmt.Errorf("in line %d | failed to part from Y: %s", i, err)
		}

		toX, err := strconv.Atoi(toParts[0])
		if err != nil {
			return nil, fmt.Errorf("in line %d | failed to part to X: %s", i, err)
		}
		toY, err := strconv.Atoi(toParts[1])
		if err != nil {
			return nil, fmt.Errorf("in line %d | failed to part to Y: %s", i, err)
		}

		if fromX > toX || fromY > toY || (fromX == toX && fromY == toY) {
			return nil, fmt.Errorf("in line %d | expecting valid 2D-shape, got out of bounds from {%d,%d} to {%d,%d}", i, fromX, fromY, toX, toY)
		}

		operations = append(operations, Operation{
			From:   Point{X: fromX, Y: fromY},
			To:     Point{X: toX, Y: toY},
			Action: action,
		})
	}

	return operations, nil
}

func determineAction(line string) (Action, string, error) {
	for _, action := range Actions {
		if strings.HasPrefix(line, string(action)) {
			return action, strings.TrimSpace(strings.TrimPrefix(line, string(action))), nil
		}
	}
	return "", "", fmt.Errorf("no action found in line %s", line)
}
