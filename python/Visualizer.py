import pygame

class Visualizer:
    def __init__(self):
        self.screen = pygame.display.set_mode((1280, 720))
        self.size = 40
        self.max = 100
    def drawList(self, myList):
        self.screen.fill((0,0,0))
        axisPoints = ((25, 700), (1255, 700), (1255, 20))

        pygame.draw.lines(self.screen, (255, 255, 255), False, axisPoints, 5)
        length = len(myList)
        for index, number in enumerate(myList):
            top = 700 - (680/self.max) * number
            pygame.draw.rect(self.screen, (0, 255, 0), pygame.Rect((1230/length) * (index + 1), top, 1230 / (self.size*2), (680/self.max) * number))

                
        pygame.display.flip()

    