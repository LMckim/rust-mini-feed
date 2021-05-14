import time
from random import choice, randint, random
symbols = [
    "ABC",
    "IUI.WT",
    "JSHH.DB.U",
    "QWEQ",
    "DFG",
    "XYS"
]
venues = [
    "OMG",
    "LYX",
    "PUR",
    "ALP",
]
def gen_line(seq:int) -> bytes:
    line = ["42="+str(seq)] 
    line.append("32="+str(round(time.time())))
    line.append("50="+choice(symbols))
    line.append("51="+str(randint(1,33)*100))
    line.append("52="+str( round( random()*randint(1,100), 2) ))
    line.append("53="+choice(venues))
    return bytes("\x02"+'\x1e'.join(line)+"\x03", 'utf8')

fp = open("test_file.dump", "wb")

for i in range(0,10000):
    fp.write(gen_line(i) + b'\n')
fp.close()