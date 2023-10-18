import pygame 
import random
import time

start = time.time()

def bubbleSort(list):
    listSorted = False
    while listSorted == False:
        check = True
        for i in range(len(list) - 1):
            if list[i] > list[i+1]:
                list[i], list[i+1] = list[i+1], list[i]
                check = False 
                drawList(list)
        if check == True: listSorted = True
    
    end = time.time()
    print(end - start)
        

def drawList(list):
    screen.fill((0,0,0))

    pygame.draw.lines(screen, (255, 255, 255), False, axisPoints, 5)
    length = len(list)
    for index, number in enumerate(list):
        top = 700 - (680/max) * number
        pygame.draw.rect(screen, (0, 255, 0), pygame.Rect((1230/length) * (index + 1), top, 1230 / (size*2), (680/max) * number))

        
    pygame.display.update()


pygame.init()
screen = pygame.display.set_mode((1280, 720))
running = True

axisPoints = ((25, 700), (1255, 700), (1255, 20))

max = 100
size = 40
sortingList = random.sample(range(1, max), size)

drawList(sortingList)
bubbleSort(sortingList)

while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

pygame.quit() 