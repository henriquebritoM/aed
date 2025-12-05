import numpy as np
import matplotlib.pyplot as plt
from scipy import stats

#   inicialização de 4 arrays com os dados de cada um dos laboratórios
lab1 = [0.888, 0.983, 1.047, 1.087, 1.125, 0.997, 1.025, 0.969, 0.898, 1.018]
lab2 = np.array([1.065, 1.226, 1.332, 0.958, 0.816, 1.015, 1.071, 0.905, 1.140, 1.051])
lab3 = np.array([1.325, 1.069, 1.219, 0.958, 0.819, 1.140, 1.222, 0.995, 0.928, 1.322])
lab4 = np.array([1.232, 1.127, 1.057, 0.897, 1.222, 1.125, 0.990, 0.875, 0.930, 0.775])

# plt.title('Laboratório 1', fontsize=20)
# plt.xlabel('Densidade Ótica', fontsize=15)
# plt.ylabel('Frequência', fontsize=15)
# plt.tick_params(labelsize=10)
# plt.hist(lab3, 4, rwidth=0.9, color='red', alpha=0.7, edgecolor='black')

# plt.show()

# Cria a figura e os eixos para o plot
fig, ax = plt.subplots(figsize=(8, 6))

# stats.probplot() calcula os quantis e plota o resultado
# 'dist="norm"' especifica a distribuição normal como referência.
# 'plot=ax' diz ao SciPy para usar o eixo criado pelo Matplotlib.
stats.probplot(lab3, dist="norm", plot=ax)

# Configurações do gráfico
ax.set_title("Gráfico Q-Q Normal")
ax.set_xlabel("Quantis Teóricos (Distribuição Normal)")
ax.set_ylabel("Quantis Amostrais (Dados Observados)")

# Exibe o gráfico
plt.grid(True)
plt.show()
