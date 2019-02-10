EESchema Schematic File Version 2
LIBS:power
LIBS:device
LIBS:transistors
LIBS:conn
LIBS:linear
LIBS:regul
LIBS:74xx
LIBS:cmos4000
LIBS:adc-dac
LIBS:memory
LIBS:xilinx
LIBS:microcontrollers
LIBS:dsp
LIBS:microchip
LIBS:analog_switches
LIBS:motorola
LIBS:texas
LIBS:intel
LIBS:audio
LIBS:interface
LIBS:digital-audio
LIBS:philips
LIBS:display
LIBS:cypress
LIBS:siliconi
LIBS:opto
LIBS:atmel
LIBS:contrib
LIBS:valves
LIBS:AUDIO-JACK-3
LIBS:LIB_MPE_BOARD
LIBS:MACE_LIB
LIBS:CarteConnecteur-cache
EELAYER 25 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L CONN_02X08 P1
U 1 1 5A9E9AC5
P 4400 5300
F 0 "P1" H 4400 5750 50  0000 C CNN
F 1 "CONN_02X08" V 4400 5300 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_2x08_Pitch2.54mm" H 4400 4100 50  0001 C CNN
F 3 "" H 4400 4100 50  0000 C CNN
F 4 "Value" H 4400 5300 60  0001 C CNN "Réf fab"
F 5 "Value" H 4400 5300 60  0001 C CNN "Code commande"
F 6 "Farnell" H 4400 5300 60  0001 C CNN "Distributeur"
F 7 "0.0" H 4400 5300 60  0001 C CNN "Prix"
	1    4400 5300
	1    0    0    -1  
$EndComp
$Comp
L CONN_02X08 P2
U 1 1 5A9E9B42
P 6700 5300
F 0 "P2" H 6700 5750 50  0000 C CNN
F 1 "CONN_02X08" V 6700 5300 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_2x08_Pitch2.54mm" H 6700 4100 50  0001 C CNN
F 3 "" H 6700 4100 50  0000 C CNN
F 4 "Value" H 6700 5300 60  0001 C CNN "Réf fab"
F 5 "Value" H 6700 5300 60  0001 C CNN "Code commande"
F 6 "Farnell" H 6700 5300 60  0001 C CNN "Distributeur"
F 7 "0.0" H 6700 5300 60  0001 C CNN "Prix"
	1    6700 5300
	1    0    0    -1  
$EndComp
$Comp
L VPP #PWR01
U 1 1 5A9E9C27
P 4100 4700
F 0 "#PWR01" H 4100 4550 50  0001 C CNN
F 1 "VPP" H 4100 4850 50  0000 C CNN
F 2 "" H 4100 4700 50  0000 C CNN
F 3 "" H 4100 4700 50  0000 C CNN
	1    4100 4700
	1    0    0    -1  
$EndComp
Text GLabel 4050 5150 0    39   Input ~ 0
USB_DM
Text GLabel 4050 5250 0    39   Input ~ 0
PA10_USB_ID
Text GLabel 4050 5350 0    39   Input ~ 0
MIDI_INPUT+
Text GLabel 4050 5450 0    39   Input ~ 0
MIDI_INPUT-
Text GLabel 4800 5650 2    39   Input ~ 0
CV_NOTE_OUTPUT
$Comp
L GND #PWR02
U 1 1 5A9E9DC0
P 4700 5800
F 0 "#PWR02" H 4700 5550 50  0001 C CNN
F 1 "GND" H 4700 5650 50  0000 C CNN
F 2 "" H 4700 5800 50  0000 C CNN
F 3 "" H 4700 5800 50  0000 C CNN
	1    4700 5800
	1    0    0    -1  
$EndComp
Text GLabel 4800 5150 2    39   Input ~ 0
USB_DP
Text GLabel 4800 5250 2    39   Input ~ 0
MIDI_OUTPUT
Text GLabel 6350 4950 0    39   Input ~ 0
OUT_STEREO_LEFT
Text GLabel 6350 5050 0    39   Input ~ 0
OUT_STEREO_RIGHT
Wire Wire Line
	4100 4950 4150 4950
Wire Wire Line
	4100 5050 4150 5050
Connection ~ 4100 5050
Connection ~ 4100 4950
Wire Wire Line
	4750 4950 4650 4950
Wire Wire Line
	4650 5050 4750 5050
Connection ~ 4750 5050
Wire Wire Line
	4150 5250 4050 5250
Wire Wire Line
	4050 5350 4150 5350
Wire Wire Line
	4150 5450 4050 5450
Wire Wire Line
	4150 5550 4050 5550
Wire Wire Line
	4050 5650 4150 5650
Wire Wire Line
	4800 5150 4650 5150
Wire Wire Line
	4800 5250 4650 5250
Wire Wire Line
	6450 4950 6350 4950
Wire Wire Line
	6350 5050 6450 5050
Wire Wire Line
	2300 1650 2450 1650
Text GLabel 2450 1650 2    39   Input ~ 0
MIDI_INPUT+
Text GLabel 1400 1650 0    39   Input ~ 0
MIDI_INPUT-
Wire Wire Line
	1500 1650 1400 1650
Text Notes 1650 1050 0    79   ~ 0
MIDI d'entrée
Wire Notes Line
	900  850  3050 850 
Wire Notes Line
	3050 850  3050 2100
Wire Notes Line
	3050 2100 900  2100
Wire Notes Line
	900  2100 900  850 
$Comp
L DIN_5 P4
U 1 1 5A9FC672
P 4150 1450
F 0 "P4" H 4150 1450 50  0000 C CNN
F 1 "DIN_5" H 4150 1300 50  0000 C CNN
F 2 "lib_robot:SD-50BV" H 4150 1450 50  0001 C CNN
F 3 "" H 4150 1450 50  0000 C CNN
F 4 "SD-50BV" H 4150 1450 60  0001 C CNN "Réf fab"
F 5 "CP-3150-ND" H 4150 1450 60  0001 C CNN "Code commande"
F 6 "Digikey" H 4150 1450 60  0001 C CNN "Distributeur"
F 7 "1.35" H 4150 1450 60  0001 C CNN "Prix"
	1    4150 1450
	-1   0    0    1   
$EndComp
Text Notes 3850 1050 0    79   ~ 0
MIDI de sortie
Wire Notes Line
	3100 850  5250 850 
Wire Notes Line
	5250 850  5250 2100
Wire Notes Line
	5250 2100 3100 2100
Wire Notes Line
	3100 2100 3100 850 
Text GLabel 3650 1650 0    39   Input ~ 0
MIDI_OUTPUT
Wire Wire Line
	3750 1650 3650 1650
Wire Wire Line
	4150 1850 4150 1900
$Comp
L GND #PWR03
U 1 1 5A9FC852
P 4150 1900
F 0 "#PWR03" H 4150 1650 50  0001 C CNN
F 1 "GND" H 4150 1750 50  0000 C CNN
F 2 "" H 4150 1900 50  0000 C CNN
F 3 "" H 4150 1900 50  0000 C CNN
	1    4150 1900
	1    0    0    -1  
$EndComp
Wire Wire Line
	4550 1650 4750 1650
$Comp
L R R1
U 1 1 5A9FC9B5
P 4750 1450
F 0 "R1" V 4830 1450 50  0000 C CNN
F 1 "51" V 4750 1450 50  0000 C CNN
F 2 "Resistors_SMD:R_0805" V 4680 1450 50  0001 C CNN
F 3 "" H 4750 1450 50  0000 C CNN
	1    4750 1450
	1    0    0    -1  
$EndComp
Wire Wire Line
	4750 1650 4750 1600
Wire Wire Line
	4750 1300 4750 1250
$Comp
L +3.3V #PWR04
U 1 1 5A9FCACA
P 4750 1250
F 0 "#PWR04" H 4750 1100 50  0001 C CNN
F 1 "+3.3V" H 4750 1390 50  0000 C CNN
F 2 "" H 4750 1250 50  0000 C CNN
F 3 "" H 4750 1250 50  0000 C CNN
	1    4750 1250
	1    0    0    -1  
$EndComp
Wire Notes Line
	3350 4400 8150 4400
Wire Notes Line
	7700 6250 3350 6250
Wire Notes Line
	3350 6250 3350 4400
$Comp
L JACK_mono J1
U 1 1 5A9FD2D1
P 6450 1200
F 0 "J1" H 6450 1075 50  0000 C CNN
F 1 "JACK_mono" H 6475 1450 50  0000 C CNN
F 2 "lib_robot:PJ-301B" H 6550 1250 50  0001 C CNN
F 3 "" H 6550 1250 50  0000 C CNN
F 4 "PJ-301B" H 6450 1200 60  0001 C CNN "Réf fab"
F 5 "Value" H 6450 1200 60  0001 C CNN "Code commande"
F 6 "Farnell" H 6450 1200 60  0001 C CNN "Distributeur"
F 7 "0.0" H 6450 1200 60  0001 C CNN "Prix"
	1    6450 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 1250 6100 1300
$Comp
L GND #PWR05
U 1 1 5A9FD514
P 6100 1300
F 0 "#PWR05" H 6100 1050 50  0001 C CNN
F 1 "GND" H 6100 1150 50  0000 C CNN
F 2 "" H 6100 1300 50  0000 C CNN
F 3 "" H 6100 1300 50  0000 C CNN
	1    6100 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 1050 6000 1050
Text GLabel 6000 1050 0    39   Input ~ 0
GATE_OUTPUT
$Comp
L JACK_mono J2
U 1 1 5A9FD927
P 6450 1800
F 0 "J2" H 6450 1675 50  0000 C CNN
F 1 "JACK_mono" H 6475 2050 50  0000 C CNN
F 2 "lib_robot:PJ-301B" H 6550 1850 50  0001 C CNN
F 3 "" H 6550 1850 50  0000 C CNN
F 4 "PJ-301B" H 6450 1800 60  0001 C CNN "Réf fab"
F 5 "Value" H 6450 1800 60  0001 C CNN "Code commande"
F 6 "Farnell" H 6450 1800 60  0001 C CNN "Distributeur"
F 7 "0.0" H 6450 1800 60  0001 C CNN "Prix"
	1    6450 1800
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 1850 6100 1900
$Comp
L GND #PWR06
U 1 1 5A9FD92E
P 6100 1900
F 0 "#PWR06" H 6100 1650 50  0001 C CNN
F 1 "GND" H 6100 1750 50  0000 C CNN
F 2 "" H 6100 1900 50  0000 C CNN
F 3 "" H 6100 1900 50  0000 C CNN
	1    6100 1900
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 1650 6000 1650
Text GLabel 6000 1650 0    39   Input ~ 0
CLOCK_OUTPUT
Wire Notes Line
	5300 850  5300 2100
Wire Notes Line
	5300 2100 7000 2100
Wire Notes Line
	7000 2100 7000 850 
Wire Notes Line
	7000 850  5300 850 
$Comp
L JACK_mono J3
U 1 1 5A9FDDFD
P 8250 1200
F 0 "J3" H 8250 1075 50  0000 C CNN
F 1 "JACK_mono" H 8275 1450 50  0000 C CNN
F 2 "lib_robot:PJ-301B" H 8350 1250 50  0001 C CNN
F 3 "" H 8350 1250 50  0000 C CNN
F 4 "PJ-301B" H 8250 1200 60  0001 C CNN "Réf fab"
F 5 "Value" H 8250 1200 60  0001 C CNN "Code commande"
F 6 "Farnell" H 8250 1200 60  0001 C CNN "Distributeur"
F 7 "0.0" H 8250 1200 60  0001 C CNN "Prix"
	1    8250 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	7900 1250 7900 1300
$Comp
L GND #PWR07
U 1 1 5A9FDE04
P 7900 1300
F 0 "#PWR07" H 7900 1050 50  0001 C CNN
F 1 "GND" H 7900 1150 50  0000 C CNN
F 2 "" H 7900 1300 50  0000 C CNN
F 3 "" H 7900 1300 50  0000 C CNN
	1    7900 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	7900 1050 7800 1050
Text GLabel 7800 1050 0    39   Input ~ 0
CV_NOTE_OUTPUT
$Comp
L JACK_mono J4
U 1 1 5A9FE0AD
P 8250 1800
F 0 "J4" H 8250 1675 50  0000 C CNN
F 1 "JACK_mono" H 8275 2050 50  0000 C CNN
F 2 "lib_robot:PJ-301B" H 8350 1850 50  0001 C CNN
F 3 "" H 8350 1850 50  0000 C CNN
F 4 "PJ-301B" H 8250 1800 60  0001 C CNN "Réf fab"
F 5 "Value" H 8250 1800 60  0001 C CNN "Code commande"
F 6 "Farnell" H 8250 1800 60  0001 C CNN "Distributeur"
F 7 "0.0" H 8250 1800 60  0001 C CNN "Prix"
	1    8250 1800
	1    0    0    -1  
$EndComp
Wire Wire Line
	7900 1850 7900 1900
$Comp
L GND #PWR08
U 1 1 5A9FE0B4
P 7900 1900
F 0 "#PWR08" H 7900 1650 50  0001 C CNN
F 1 "GND" H 7900 1750 50  0000 C CNN
F 2 "" H 7900 1900 50  0000 C CNN
F 3 "" H 7900 1900 50  0000 C CNN
	1    7900 1900
	1    0    0    -1  
$EndComp
Wire Wire Line
	7900 1650 7800 1650
Text GLabel 7800 1650 0    39   Input ~ 0
CV_VELOCITE_OUTPUT
Wire Notes Line
	7050 850  7050 2100
Wire Notes Line
	7050 2100 8800 2100
Wire Notes Line
	8800 2100 8800 850 
Wire Notes Line
	8800 850  7050 850 
$Comp
L JACK_mono J5
U 1 1 5A9FE824
P 10050 1200
F 0 "J5" H 10050 1075 50  0000 C CNN
F 1 "JACK_mono" H 10075 1450 50  0000 C CNN
F 2 "lib_robot:PJ-301B" H 10150 1250 50  0001 C CNN
F 3 "" H 10150 1250 50  0000 C CNN
F 4 "PJ-301B" H 10050 1200 60  0001 C CNN "Réf fab"
F 5 "Value" H 10050 1200 60  0001 C CNN "Code commande"
F 6 "Farnell" H 10050 1200 60  0001 C CNN "Distributeur"
F 7 "0.0" H 10050 1200 60  0001 C CNN "Prix"
	1    10050 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	9700 1250 9700 1300
$Comp
L GND #PWR09
U 1 1 5A9FE82B
P 9700 1300
F 0 "#PWR09" H 9700 1050 50  0001 C CNN
F 1 "GND" H 9700 1150 50  0000 C CNN
F 2 "" H 9700 1300 50  0000 C CNN
F 3 "" H 9700 1300 50  0000 C CNN
	1    9700 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	9700 1050 9600 1050
Text GLabel 9600 1050 0    39   Input ~ 0
CV_n3_OUTPUT
$Comp
L JACK_mono J6
U 1 1 5A9FE833
P 10050 1800
F 0 "J6" H 10050 1675 50  0000 C CNN
F 1 "JACK_mono" H 10075 2050 50  0000 C CNN
F 2 "lib_robot:PJ-301B" H 10150 1850 50  0001 C CNN
F 3 "" H 10150 1850 50  0000 C CNN
F 4 "PJ-301B" H 10050 1800 60  0001 C CNN "Réf fab"
F 5 "Value" H 10050 1800 60  0001 C CNN "Code commande"
F 6 "Farnell" H 10050 1800 60  0001 C CNN "Distributeur"
F 7 "0.0" H 10050 1800 60  0001 C CNN "Prix"
	1    10050 1800
	1    0    0    -1  
$EndComp
Wire Wire Line
	9700 1850 9700 1900
$Comp
L GND #PWR010
U 1 1 5A9FE83A
P 9700 1900
F 0 "#PWR010" H 9700 1650 50  0001 C CNN
F 1 "GND" H 9700 1750 50  0000 C CNN
F 2 "" H 9700 1900 50  0000 C CNN
F 3 "" H 9700 1900 50  0000 C CNN
	1    9700 1900
	1    0    0    -1  
$EndComp
Wire Wire Line
	9700 1650 9600 1650
Text GLabel 9600 1650 0    39   Input ~ 0
CV_n4_OUTPUT
Wire Notes Line
	8850 850  8850 2100
Wire Notes Line
	8850 2100 10600 2100
Wire Notes Line
	10600 2100 10600 850 
Wire Notes Line
	10600 850  8850 850 
$Comp
L JACK_stereo J7
U 1 1 5A9FF652
P 2100 2750
F 0 "J7" H 2100 2625 50  0000 C CNN
F 1 "JACK_stereo" H 2150 3050 50  0000 C CNN
F 2 "lib_robot:PJ301CM" H 2200 2875 50  0001 C CNN
F 3 "" H 2200 2875 50  0000 C CNN
F 4 "PJ301CM" H 2100 2750 60  0001 C CNN "Réf fab"
F 5 "Value" H 2100 2750 60  0001 C CNN "Code commande"
F 6 "www.thonk.co.uk" H 2100 2750 60  0001 C CNN "Distributeur"
F 7 "0.0" H 2100 2750 60  0001 C CNN "Prix"
	1    2100 2750
	1    0    0    -1  
$EndComp
Text GLabel 1700 2650 0    39   Input ~ 0
OUT_STEREO_LEFT
Text GLabel 1700 2850 0    39   Input ~ 0
OUT_STEREO_RIGHT
Wire Wire Line
	1800 2850 1700 2850
Wire Notes Line
	3050 2200 900  2200
Wire Notes Line
	900  2200 900  3400
Wire Notes Line
	900  3400 3050 3400
Wire Notes Line
	3050 3400 3050 2200
$Comp
L Hole P5
U 1 1 5AB3BD51
P 1400 4850
F 0 "P5" H 1400 4950 50  0000 C CNN
F 1 "Hole" V 1500 4850 50  0000 C CNN
F 2 "lib_robot:Hole_2.5mm" H 1400 4850 60  0001 C CNN
F 3 "" H 1400 4850 60  0000 C CNN
F 4 "Value" H 1400 4850 60  0001 C CNN "Réf fab"
F 5 "Value" H 1400 4850 60  0001 C CNN "Code commande"
F 6 "Farnell" H 1400 4850 60  0001 C CNN "Distributeur"
F 7 "0.0" H 1400 4850 60  0001 C CNN "Prix"
	1    1400 4850
	1    0    0    -1  
$EndComp
$Comp
L Hole P6
U 1 1 5AB3C112
P 1400 5100
F 0 "P6" H 1400 5200 50  0000 C CNN
F 1 "Hole" V 1500 5100 50  0000 C CNN
F 2 "lib_robot:Hole_2.5mm" H 1400 5100 60  0001 C CNN
F 3 "" H 1400 5100 60  0000 C CNN
F 4 "Value" H 1400 5100 60  0001 C CNN "Réf fab"
F 5 "Value" H 1400 5100 60  0001 C CNN "Code commande"
F 6 "Farnell" H 1400 5100 60  0001 C CNN "Distributeur"
F 7 "0.0" H 1400 5100 60  0001 C CNN "Prix"
	1    1400 5100
	1    0    0    -1  
$EndComp
$Comp
L Hole P7
U 1 1 5AB3C14A
P 1400 5350
F 0 "P7" H 1400 5450 50  0000 C CNN
F 1 "Hole" V 1500 5350 50  0000 C CNN
F 2 "lib_robot:Hole_2.5mm" H 1400 5350 60  0001 C CNN
F 3 "" H 1400 5350 60  0000 C CNN
F 4 "Value" H 1400 5350 60  0001 C CNN "Réf fab"
F 5 "Value" H 1400 5350 60  0001 C CNN "Code commande"
F 6 "Farnell" H 1400 5350 60  0001 C CNN "Distributeur"
F 7 "0.0" H 1400 5350 60  0001 C CNN "Prix"
	1    1400 5350
	1    0    0    -1  
$EndComp
$Comp
L Hole P8
U 1 1 5AB3C18D
P 1400 5550
F 0 "P8" H 1400 5650 50  0000 C CNN
F 1 "Hole" V 1500 5550 50  0000 C CNN
F 2 "lib_robot:Hole_2.5mm" H 1400 5550 60  0001 C CNN
F 3 "" H 1400 5550 60  0000 C CNN
F 4 "Value" H 1400 5550 60  0001 C CNN "Réf fab"
F 5 "Value" H 1400 5550 60  0001 C CNN "Code commande"
F 6 "Farnell" H 1400 5550 60  0001 C CNN "Distributeur"
F 7 "0.0" H 1400 5550 60  0001 C CNN "Prix"
	1    1400 5550
	1    0    0    -1  
$EndComp
Wire Wire Line
	1200 4850 1200 5700
Connection ~ 1200 5100
Connection ~ 1200 5350
Connection ~ 1200 5550
$Comp
L GND #PWR011
U 1 1 5AB3C7D1
P 1200 5700
F 0 "#PWR011" H 1200 5450 50  0001 C CNN
F 1 "GND" H 1200 5550 50  0000 C CNN
F 2 "" H 1200 5700 50  0000 C CNN
F 3 "" H 1200 5700 50  0000 C CNN
	1    1200 5700
	1    0    0    -1  
$EndComp
Text Notes 4350 4650 0    79   ~ 0
VPP = +12V
Wire Wire Line
	4650 5350 4800 5350
Text GLabel 4800 5350 2    39   Input ~ 0
GATE_OUTPUT
Wire Wire Line
	4750 5050 4750 4950
Wire Wire Line
	4100 4700 4100 5050
Wire Wire Line
	4650 5450 4700 5450
Wire Wire Line
	4700 5450 4700 5800
Wire Wire Line
	4800 5650 4650 5650
Wire Wire Line
	4150 5150 4050 5150
Text GLabel 4050 5550 0    39   Input ~ 0
CLOCK_OUTPUT
Text GLabel 4050 5650 0    39   Input ~ 0
CV_VELOCITE_OUTPUT
Wire Wire Line
	6450 5150 6400 5150
Wire Wire Line
	6400 5150 6400 5800
Wire Wire Line
	6450 5450 6400 5450
Connection ~ 6400 5450
$Comp
L GND #PWR012
U 1 1 5AD5D293
P 6400 5800
F 0 "#PWR012" H 6400 5550 50  0001 C CNN
F 1 "GND" H 6400 5650 50  0000 C CNN
F 2 "" H 6400 5800 50  0000 C CNN
F 3 "" H 6400 5800 50  0000 C CNN
	1    6400 5800
	1    0    0    -1  
$EndComp
Text GLabel 7050 5550 2    39   Input ~ 0
CV_n3_OUTPUT
Text GLabel 7050 5650 2    39   Input ~ 0
CV_n4_OUTPUT
Text GLabel 7050 5450 2    39   Input ~ 0
AUDIO_STEREO_IN_LEFT
Wire Wire Line
	7050 5450 6950 5450
Wire Wire Line
	6950 5550 7050 5550
Wire Wire Line
	7050 5650 6950 5650
Text GLabel 6350 5350 0    39   Input ~ 0
AUDIO_STEREO_IN_RIGHT
Wire Wire Line
	6450 5350 6350 5350
Wire Wire Line
	6450 5550 6350 5550
Wire Wire Line
	6450 5650 6350 5650
Text GLabel 6350 5550 0    39   Input ~ 0
CV_IN_LEFT
Text GLabel 6350 5650 0    39   Input ~ 0
CV_IN_RIGHT
Wire Notes Line
	7650 6250 8150 6250
Wire Notes Line
	8150 6250 8150 4400
$Comp
L DIN_5 P3
U 1 1 5A9E9F69
P 1900 1450
F 0 "P3" H 1900 1450 50  0000 C CNN
F 1 "DIN_5" H 1900 1300 50  0000 C CNN
F 2 "lib_robot:SD-50BV" H 1900 1450 50  0001 C CNN
F 3 "" H 1900 1450 50  0000 C CNN
F 4 "SD-50BV" H 1900 1450 60  0001 C CNN "Réf fab"
F 5 "CP-3150-ND " H 1900 1450 60  0001 C CNN "Code commande"
F 6 "Digikey" H 1900 1450 60  0001 C CNN "Distributeur"
F 7 "1.35" H 1900 1450 60  0001 C CNN "Prix"
	1    1900 1450
	-1   0    0    1   
$EndComp
Wire Wire Line
	1800 2650 1700 2650
$Comp
L USB_OTG P9
U 1 1 5AD63B5E
P 4300 2750
F 0 "P9" H 4625 2625 50  0000 C CNN
F 1 "USB_OTG" H 4300 2950 50  0000 C CNN
F 2 "lib_robot:USB_Micro-B-verticale" V 4250 2650 50  0001 C CNN
F 3 "" V 4250 2650 50  0000 C CNN
F 4 "105133-0011 " H 4300 2750 60  0001 C CNN "Réf fab"
F 5 "2614949" H 4300 2750 60  0001 C CNN "Code commande"
F 6 "Farnell" H 4300 2750 60  0001 C CNN "Distributeur"
F 7 "0.0" H 4300 2750 60  0001 C CNN "Prix"
	1    4300 2750
	0    1    1    0   
$EndComp
Text GLabel 3900 2650 0    39   Input ~ 0
USB_DM
Text GLabel 3900 2750 0    39   Input ~ 0
USB_DP
Wire Wire Line
	4000 2650 3900 2650
Wire Wire Line
	3900 2750 4000 2750
Wire Wire Line
	4000 2950 3950 2950
Wire Wire Line
	3950 2950 3950 3050
$Comp
L GND #PWR013
U 1 1 5AD64F57
P 3950 3050
F 0 "#PWR013" H 3950 2800 50  0001 C CNN
F 1 "GND" H 3950 2900 50  0000 C CNN
F 2 "" H 3950 3050 50  0000 C CNN
F 3 "" H 3950 3050 50  0000 C CNN
	1    3950 3050
	1    0    0    -1  
$EndComp
Text GLabel 3900 2850 0    39   Input ~ 0
PA10_USB_ID
Wire Wire Line
	4000 2850 3900 2850
$Comp
L JACK_stereo J8
U 1 1 5AD70A22
P 2100 4000
F 0 "J8" H 2100 3875 50  0000 C CNN
F 1 "JACK_stereo" H 2150 4300 50  0000 C CNN
F 2 "lib_robot:PJ301CM" H 2200 4125 50  0001 C CNN
F 3 "" H 2200 4125 50  0000 C CNN
F 4 "PJ301CM" H 2100 4000 60  0001 C CNN "Réf fab"
F 5 "Value" H 2100 4000 60  0001 C CNN "Code commande"
F 6 "www.thonk.co.uk" H 2100 4000 60  0001 C CNN "Distributeur"
F 7 "0.0" H 2100 4000 60  0001 C CNN "Prix"
	1    2100 4000
	1    0    0    -1  
$EndComp
Text GLabel 1700 3900 0    39   Input ~ 0
OUT_STEREO_LEFT
Text GLabel 1700 4100 0    39   Input ~ 0
OUT_STEREO_RIGHT
Wire Wire Line
	1800 4100 1700 4100
Wire Notes Line
	3050 3450 900  3450
Wire Notes Line
	900  3450 900  4650
Wire Notes Line
	900  4650 3050 4650
Wire Notes Line
	3050 4650 3050 3450
Wire Wire Line
	1800 3900 1700 3900
$Comp
L JACK_mono J9
U 1 1 5AD70EA4
P 6450 2550
F 0 "J9" H 6450 2425 50  0000 C CNN
F 1 "JACK_mono" H 6475 2800 50  0000 C CNN
F 2 "lib_robot:PJ-301B" H 6550 2600 50  0001 C CNN
F 3 "" H 6550 2600 50  0000 C CNN
F 4 "PJ-301B" H 6450 2550 60  0001 C CNN "Réf fab"
F 5 "Value" H 6450 2550 60  0001 C CNN "Code commande"
F 6 "Farnell" H 6450 2550 60  0001 C CNN "Distributeur"
F 7 "0.0" H 6450 2550 60  0001 C CNN "Prix"
	1    6450 2550
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 2600 6100 2650
$Comp
L GND #PWR014
U 1 1 5AD70EAB
P 6100 2650
F 0 "#PWR014" H 6100 2400 50  0001 C CNN
F 1 "GND" H 6100 2500 50  0000 C CNN
F 2 "" H 6100 2650 50  0000 C CNN
F 3 "" H 6100 2650 50  0000 C CNN
	1    6100 2650
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 2400 6000 2400
Text GLabel 6000 2400 0    39   Input ~ 0
GATE_OUTPUT
Wire Notes Line
	3100 2200 5250 2200
Wire Notes Line
	5250 2200 5250 3400
Wire Notes Line
	5250 3400 3100 3400
Wire Notes Line
	3100 3400 3100 2200
Text GLabel 4800 5550 2    39   Input ~ 0
VUSB_5V
Wire Wire Line
	4800 5550 4650 5550
Text GLabel 3900 2550 0    39   Input ~ 0
VUSB_5V
Wire Wire Line
	4000 2550 3900 2550
$Comp
L GND #PWR015
U 1 1 5AD75CD2
P 4400 3150
F 0 "#PWR015" H 4400 2900 50  0001 C CNN
F 1 "GND" H 4400 3000 50  0000 C CNN
F 2 "" H 4400 3150 50  0000 C CNN
F 3 "" H 4400 3150 50  0000 C CNN
	1    4400 3150
	1    0    0    -1  
$EndComp
Wire Notes Line
	5300 2200 7000 2200
Wire Notes Line
	7000 2200 7000 3400
Wire Notes Line
	7000 3400 5300 3400
Wire Notes Line
	5300 3400 5300 2200
$EndSCHEMATC
