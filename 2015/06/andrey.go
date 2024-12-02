package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type light struct {
	stat int
	// x    int
	// y    int
}

type command struct {
	num    int
	act    string
	x_init int
	y_init int
	x_fin  int
	y_fin  int
}

var grid [1000][1000]light
var instr [300]string
var instr_proc [300]command
var act [3]string = [3]string{"turn onn", "turn off", "toggle"}
var a string
var b []string
var c1 []string
var c2 []string
var actt string
var x_init int
var y_init int
var x_fin int
var y_fin int
var cntt int

const elemCnt = 300

func main() {

	//fmt.Println(processing ())

	grid = processing()

	cntt = 0
	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			if grid[j][i].stat == 1 {
				cntt++
			}
		}
	}
	fmt.Println(cntt)
	writeToFile(grid)

	//fmt.Println(fileCntRows())
	//fmt.Println(len(fileToSlice()))
	//writeToFile()

	// fmt.Println(grid[0][501])
	//fmt.Println(grid[0][0])
	//fmt.Println(grid[1][0])
	// fmt.Println(grid[0])
	// fmt.Println(len(grid)*len(grid[0]))
	//fmt.Println(fileToArray())

	//fmt.Println(instr)
	//fmt.Println(instr_proc)

}

func writeToFile(grid [1000][1000]light) {
	// Открываем файл на запись
	file, err := os.Create("C:\\!AB\\prog\\2015 6\\output.txt")
	if err != nil {
		fmt.Println("Ошибка при создании файла:", err)
		return
	}
	defer file.Close()

	// Создаем буферизованного писателя
	writer := bufio.NewWriter(file)

	// Записываем числа в файл построчно
	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			// Преобразуем число в строку
			line := fmt.Sprintf("%d", grid[i][j].stat)
			_, err := writer.WriteString(line)
			if err != nil {
				fmt.Println("Ошибка при записи строки:", err)
			}

		}
		_, err = writer.WriteString("\n")
	}


	// Флушим буфер и закрываем файл
	err = writer.Flush()
	if err != nil {
		fmt.Println("Ошибка при флушировании буфера:", err)
	}
}

func processing() [1000][1000]light {
	start := time.Now()

	a := make([1000][1000]light)

	for i := 0; i < 1000; i++ { //создаем поле
		for j := 0; j < 1000; j++ {
			grid[j][i] = light{
				stat: 0,
				// x:    j,
				// y:    i,
			}
		}
	}

	instr := fileToArray()

	for i := range instr {
		//for i := 0; i < 1; i++ {
		for j := range act {
			if strings.Contains(instr[i], act[j]) {
				instr_proc[i].act = act[j]
				a = strings.TrimSpace(
					strings.Replace(
						strings.Replace(
							instr[i], act[j], "", 1), "through", "", 1))
				//fmt.Println(a)
			}
		}
		b = strings.Split(a, " ")
		c1 = strings.Split(b[0], ",")
		c2 = strings.Split(b[2], ",")
		instr_proc[i].num = i + 1
		instr_proc[i].x_init = stringToInt(c1[0])
		instr_proc[i].y_init = stringToInt(c1[1])
		instr_proc[i].x_fin = stringToInt(c2[0])
		instr_proc[i].y_fin = stringToInt(c2[1])
	}

	//{1 980 775}
	//{135 turn off 306 44 457 444}

	//var instr_proc_t [3]command = [3]command{{1, "turn off", 0, 0, 3, 3}, {1, "turn on", 0, 0, 1, 1}, {1, "toggle", 0, 0, 2, 2}}

	//var instr_proc_t [3]command = [3]command{{1, "turn on", 0, 0, 999, 999}, {1, "toggle", 0, 0, 999, 0}, {1, "turn off", 499, 499, 500, 500}}
	for idx := 0; idx < 300; idx++ {
		//for i := range instr_proc {

		actt = instr_proc[idx].act
		x_init = instr_proc[idx].x_init
		x_fin = instr_proc[idx].x_fin
		y_init = instr_proc[idx].y_init
		y_fin = instr_proc[idx].y_fin

		fmt.Println(instr_proc[idx])

		// actt = instr_proc_t[idx].act
		// x_init = instr_proc_t[idx].x_init
		// x_fin = instr_proc_t[idx].x_fin
		// y_init = instr_proc_t[idx].y_init
		// y_fin = instr_proc_t[idx].y_fin

		// fmt.Println(instr_proc_t[idx])

		if actt == "turn on" {
			for i := x_init; i <= x_fin; i++ {
				for j := y_init; j <= y_fin; j++ {
					grid[j][i].stat = 1
					// fmt.Println("ON", grid[j][i])
				}

			}

			// cntt = 0
			// for i := 0; i < 1000; i++ {
			// 	for j := 0; j < 1000; j++ {
			// 		if grid[j][i].stat == 1 {
			// 			cntt++
			// 		}
			// 	}
			// }
			// fmt.Println(cntt)

		} else if actt == "turn off" {
			for i := x_init; i <= x_fin; i++ {
				for j := y_init; j <= y_fin; j++ {
					grid[j][i].stat = 0
					// fmt.Println("OFF", grid[j][i])
				}
			}

			// 	cntt = 0
			// 	for i := 0; i < 1000; i++ {
			// 		for j := 0; j < 1000; j++ {
			// 			if grid[j][i].stat == 1 {
			// 				cntt++
			// 			}
			// 		}
			// 	}
			// 	fmt.Println(cntt)
		} else if actt == "toggle" {
			for i := x_init; i <= x_fin; i++ {
				for j := y_init; j <= y_fin; j++ {
					if grid[j][i].stat == 0 {
						grid[j][i].stat = 1
					} else if grid[j][i].stat == 1 {
						grid[j][i].stat = 0
					}
					// fmt.Println("OFF", grid[j][i])
				}
			}
		}
	}

	end := time.Now()

	elapsed := end.Sub(start)
	fmt.Println("Время выполнения:", elapsed)

	return grid
}

func stringToInt(s string) int {
	num, err := strconv.Atoi(s)
	if err != nil {
		panic("Ошибка ковертации")
	} else {
		return num
	}

}

func fileCntRows() (cnt int) { //нужно посчиать кол-во строк в исходнике
	// Открываем файл
	file, err := os.Open("C:\\!AB\\prog\\2015 6\\2015-6.txt")
	if err != nil {
		fmt.Println("Ошибка при открытии файла:", err)
		return
	}
	// Создаем сканер для построчного чтения
	scanner := bufio.NewScanner(file)
	// Читаем файл построчно
	for scanner.Scan() {
		cnt++
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Ошибка при сканировании:", err)
	}
	file.Close()
	return cnt
}

func fileToArray() (array [elemCnt]string) { //создаем массив нужно задать размер его сначала
	// Открываем файл
	file, err := os.Open("C:\\!AB\\prog\\2015 6\\2015-6.txt")
	if err != nil {
		fmt.Println("Ошибка при открытии файла:", err)
		return
	}

	// Создаем сканер для построчного чтения
	scanner := bufio.NewScanner(file)

	// Читаем файл построчно
	var arr [elemCnt]string
	var cnt int
	cnt = 0
	for scanner.Scan() {
		line := scanner.Text()
		arr[cnt] = line
		cnt++
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Ошибка при сканировании:", err)
	}

	file.Close()

	return arr

}

====================2
package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
	"time"
)

type light struct {
	stat int
	// x    int
	// y    int
}

type command struct {
	num    int
	act    string
	x_init int
	y_init int
	x_fin  int
	y_fin  int
}

var grid [1000][1000]light
var instr [300]string
var instr_proc [300]command
var act [3]string = [3]string{"turn on", "turn off", "toggle"}
var a string
var b []string
var c1 []string
var c2 []string
var actt string
var x_init int
var y_init int
var x_fin int
var y_fin int
var cntt int

const elemCnt = 300

func main() {

	//fmt.Println(processing ())

	grid = processing()

	cntt = 0
	for i := 0; i < 1000; i++ {
		for j := 0; j < 1000; j++ {
			if grid[j][i].stat != 0 {
				cntt += grid[j][i].stat
			}
		}

	}
	fmt.Println(cntt)

	//fmt.Println(fileCntRows())
	//fmt.Println(len(fileToSlice()))
	//writeToFile()

	// fmt.Println(grid[0][501])
	//fmt.Println(grid[0][0])
	//fmt.Println(grid[1][0])
	// fmt.Println(grid[0])
	// fmt.Println(len(grid)*len(grid[0]))
	//fmt.Println(fileToArray())

	//fmt.Println(instr)
	//fmt.Println(instr_proc)

}

func processing() [1000][1000]light {
	start := time.Now()

	for i := 0; i < 1000; i++ { //создаем поле
		for j := 0; j < 1000; j++ {
			grid[j][i] = light{
				stat: 0,
				// x:    j,
				// y:    i,
			}
		}
	}

	instr := fileToArray()

	for i := range instr {
		//for i := 0; i < 1; i++ {
		for j := range act {
			if strings.Contains(instr[i], act[j]) {
				instr_proc[i].act = act[j]
				a = strings.TrimSpace(
					strings.Replace(
						strings.Replace(
							instr[i], act[j], "", 1), "through", "", 1))
				//fmt.Println(a)
			}
		}
		b = strings.Split(a, " ")
		c1 = strings.Split(b[0], ",")
		c2 = strings.Split(b[2], ",")
		instr_proc[i].num = i + 1
		instr_proc[i].x_init = stringToInt(c1[0])
		instr_proc[i].y_init = stringToInt(c1[1])
		instr_proc[i].x_fin = stringToInt(c2[0])
		instr_proc[i].y_fin = stringToInt(c2[1])
	}

	//{1 980 775}
	//{135 turn off 306 44 457 444}

	//var instr_proc_t [3]command = [3]command{{1, "turn off", 0, 0, 3, 3}, {1, "turn on", 0, 0, 1, 1}, {1, "toggle", 0, 0, 2, 2}}

	//var instr_proc_t [2]command = [2]command{{1, "turn on", 0, 0, 0, 0}, {2, "toggle", 0, 0, 999, 999}}
	for idx := 0; idx < 300; idx++ {
		//for i := range instr_proc {

		actt = instr_proc[idx].act
		x_init = instr_proc[idx].x_init
		x_fin = instr_proc[idx].x_fin
		y_init = instr_proc[idx].y_init
		y_fin = instr_proc[idx].y_fin

		fmt.Println(instr_proc[idx])

		// actt = instr_proc_t[idx].act
		// x_init = instr_proc_t[idx].x_init
		// x_fin = instr_proc_t[idx].x_fin
		// y_init = instr_proc_t[idx].y_init
		// y_fin = instr_proc_t[idx].y_fin

		// fmt.Println(instr_proc_t[idx])

		if actt == "turn on" {
			for i := x_init; i <= x_fin; i++ {
				for j := y_init; j <= y_fin; j++ {
					grid[j][i].stat += 1
					// fmt.Println("ON", grid[j][i])
				}

			}

			// cntt = 0
			// for i := 0; i < 1000; i++ {
			// 	for j := 0; j < 1000; j++ {
			// 		if grid[j][i].stat == 1 {
			// 			cntt++
			// 		}
			// 	}
			// }
			// fmt.Println(cntt)

		} else if actt == "turn off" {
			for i := x_init; i <= x_fin; i++ {
				for j := y_init; j <= y_fin; j++ {
					if grid[j][i].stat > 0 {
						grid[j][i].stat -= 1
					} else {
						grid[j][i].stat = 0
					}
					// fmt.Println("OFF", grid[j][i])
				}
			}

			// 	cntt = 0
			// 	for i := 0; i < 1000; i++ {
			// 		for j := 0; j < 1000; j++ {
			// 			if grid[j][i].stat == 1 {
			// 				cntt++
			// 			}
			// 		}
			// 	}
			// 	fmt.Println(cntt)
		} else if actt == "toggle" {
			for i := x_init; i <= x_fin; i++ {
				for j := y_init; j <= y_fin; j++ {
					grid[j][i].stat += 2
					// fmt.Println("OFF", grid[j][i])
				}
			}
		}
	}

	end := time.Now()

	elapsed := end.Sub(start)
	fmt.Println("Время выполнения:", elapsed)

	return grid
}

func stringToInt(s string) int {
	num, err := strconv.Atoi(s)
	if err != nil {
		panic("Ошибка ковертации")
	} else {
		return num
	}

}

func fileCntRows() (cnt int) { //нужно посчиать кол-во строк в исходнике
	// Открываем файл
	file, err := os.Open("C:\\!AB\\prog\\2015 6\\2015-6.txt")
	if err != nil {
		fmt.Println("Ошибка при открытии файла:", err)
		return
	}
	// Создаем сканер для построчного чтения
	scanner := bufio.NewScanner(file)
	// Читаем файл построчно
	for scanner.Scan() {
		cnt++
	}
	if err := scanner.Err(); err != nil {
		fmt.Println("Ошибка при сканировании:", err)
	}
	file.Close()
	return cnt
}

func fileToArray() (array [elemCnt]string) { //создаем массив нужно задать размер его сначала
	// Открываем файл
	file, err := os.Open("C:\\!AB\\prog\\2015 6\\2015-6.txt")
	if err != nil {
		fmt.Println("Ошибка при открытии файла:", err)
		return
	}

	// Создаем сканер для построчного чтения
	scanner := bufio.NewScanner(file)

	// Читаем файл построчно
	var arr [elemCnt]string
	var cnt int
	cnt = 0
	for scanner.Scan() {
		line := scanner.Text()
		arr[cnt] = line
		cnt++
	}

	if err := scanner.Err(); err != nil {
		fmt.Println("Ошибка при сканировании:", err)
	}

	file.Close()


	return arr

}
