import pygame
import random
from BubbleSort import BubbleSort
from Visualizer import Visualizer

def main():
    pygame.init()
    running = True

    max_value = 100
    size = 40

    sortingList = random.sample(range(1, max_value), size)

    visualizer = Visualizer()
    visualizer.drawList(sortingList)

    algorithm = BubbleSort(sortingList)
    algorithm.visualizator = visualizer
    algorithm.sort()

    while running:
        for event in pygame.event.get():
            if event.type == pygame.QUIT:
                running = False

    pygame.quit()

if __name__ == "__main__":
    main()
