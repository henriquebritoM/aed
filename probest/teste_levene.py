import numpy as np
from scipy.stats import levene

#   Teste para verificar a suposição de homogeneidade de variâncias -
#   homocedasticidade - entre dois ou mais grupos independentes
#   É um pressuposto comum para testes paramétricos como a ANOVA

#   Esse script testa a suposta homogeneidade para cada amostra

# Dados de exemplo para os 4 laboratórios
lab1 = [0.888, 0.983, 1.047, 1.087, 1.125, 0.997, 1.025, 0.969, 0.898, 1.018]
lab2 = [1.065, 1.226, 1.332, 0.958, 0.816, 1.015, 1.071, 0.905, 1.140, 1.051]
lab3 = [1.325, 1.069, 1.219, 0.958, 0.819, 1.140, 1.222, 0.995, 0.928, 1.322]
lab4 = [1.232, 1.127, 1.057, 0.897, 1.222, 1.125, 0.990, 0.875, 0.930, 0.775]

#   Chamamos a função levene
#   ela retorna o valor W e o p-value, para nós apenas o p-value será relevante
estatistica_W, valor_p = levene(lab1, lab2, lab3, lab4, center='mean')

# Interpretação
alpha = 0.05

print("As amostras são homogêneas?", valor_p > alpha)