import numpy as np
from scipy.stats import shapiro

#   Teste de normalizade para as amostras
#   O de hipótese anova simples tem como pré-requisito que
#   suas amostras sigam um distribuição normal

#   esse script testa essa hipótese para cada amostra
#   para isso, estamos usando o teste de normalidade Shapiro Wilk

#   inicialização de 4 arrays com os dados de cada um dos laboratórios
lab1 = np.array([0.888, 0.983, 1.047, 1.087, 1.125, 0.997, 1.025, 0.969, 0.898, 1.018])
lab2 = np.array([1.065, 1.226, 1.332, 0.958, 0.816, 1.015, 1.071, 0.905, 1.140, 1.051])
lab3 = np.array([1.325, 1.069, 1.219, 0.958, 0.819, 1.140, 1.222, 0.995, 0.928, 1.322])
lab4 = np.array([1.232, 1.127, 1.057, 0.897, 1.222, 1.125, 0.990, 0.875, 0.930, 0.775])

#   Chamamos a função shapiro
#   ela retorna o valor W e o p-value, para nós apenas o p-value será relevante
valo_w1, valor_p1 = shapiro(lab1)
valo_w2, valor_p2 = shapiro(lab2)
valo_w3, valor_p3 = shapiro(lab3)
valo_w4, valor_p4 = shapiro(lab4)

#   Um p-value acima de 0.5 indica que a população da amostra segue uma distribuição normal
normal1 = valor_p1 > 0.5
normal2 = valor_p2 > 0.5
normal3 = valor_p3 > 0.5
normal4 = valor_p4 > 0.5

print("As amostras seguem a distribuição normal? ", normal1 & normal2 & normal3 & normal4)
