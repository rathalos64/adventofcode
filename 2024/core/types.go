package core

import "time"

type Exercise interface {
	Task1() (int, time.Duration, error)
	Task2() (int, time.Duration, error)

	GetInput() string
	GetDescription() string
}
