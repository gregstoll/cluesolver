import time
import clueengine

start = time.time()
(ce, s) = clueengine.ClueEngine.loadFromString("63A-.3-A.3-A.3-A.3-A.3-A.3-A.")

for i in range(100):
    simData = ce.getSimulationData()
    print(simData['Conservatory'])
end = time.time()
print(end - start)