import numpy as np
import matplotlib.pyplot as plt
from scipy import stats

#   inicialização de 4 arrays com os dados de cada um dos laboratórios
lab1 = [0.888, 0.983, 1.047, 1.087, 1.125, 0.997, 1.025, 0.969, 0.898, 1.018]
lab2 = np.array([1.065, 1.226, 1.332, 0.958, 0.816, 1.015, 1.071, 0.905, 1.140, 1.051])
lab3 = np.array([1.325, 1.069, 1.219, 0.958, 0.819, 1.140, 1.222, 0.995, 0.928, 1.322])
lab4 = np.array([1.232, 1.127, 1.057, 0.897, 1.222, 1.125, 0.990, 0.875, 0.930, 0.775])

# Cria a figura e os eixos para o plot
fig, ax = plt.subplots(figsize=(8, 6))

# stats.probplot() calcula os quantis e plota o resultado
# 'dist="norm"' especifica a distribuição normal como referência.
# 'plot=ax' diz ao SciPy para usar o eixo criado pelo Matplotlib.
stats.probplot(lab4, dist="norm", plot=ax)

# Configurações do gráfico
ax.set_title("Gráfico Q-Q Normal Lab4", fontsize=20)
ax.set_xlabel("Quantis Teóricos (Distribuição Normal)", fontsize=15)
ax.set_ylabel("Quantis Amostrais (Dados Observados)", fontsize=15)

# Exibe o gráfico
plt.grid(True)
plt.show()
