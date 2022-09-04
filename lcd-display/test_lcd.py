from rpi_lcd import LCD
from time import sleep

lcd = LCD(bus=2)

lcd.text('Hello World!', 1)
lcd.text('Raspberry Pi', 2)
lcd.text('is really', 3, 'center')
lcd.text('awesome', 4, 'right')

sleep(5)
lcd.clear()

