from re import compile as re
from sys import argv
from random import choice,seed;seed();del seed
from itertools import izip as zip
k=re(r"^::=.+",8).sub
G={}
try:
	input=raw_input
	range=xrange
except:pass
def Rue(f,A="",s=0):
	if f in G:f=G[f]
	else:
		c=f
		f=k("",open(f+".txt").read())
		if f.startswith("import "):
			a=f.find("\n",8)
			f=open(f[7:a]+".txt").read()+f[a:]
		a=f.find("\nimport ")+1
		while a:
			b=f.find("\n",a+8)
			f=f[:a]+open(f[a+7:b]+".txt").read()+f[b:]
			a=f.find("\nimport ",a)+1
		f=f.split("\n::=\n")
		G[c]=f
	c=""#::="
	R=[]
	for lf,C in zip(range(len(f)-1,-1,-1),f):
		R+=([re(R[0],16).sub,R[1] if len(R) is 2 else "" if len(R) is 1 else R[1:],len(R) is 1] for R in (R.split("::=") for R in c.split("\n") if R))
		while 1:
			while 1:
				c=C=C.replace("@@",A)
				for p0,p1,p2 in R:
					C=p0(choice(p1) if p2 else p1,C,1)
					if c is not C:break
				else:break
				if D:print(" "*s+C)
			if lf:break
			a=C.find("}")
			if a is -1:break
			while 1:
				b=C.rfind("{",0,a)
				c=C.rfind(":",0,b)
				f=C[c+1:b]
				b=C[b+1:a]
				C=C[:c]+(Smod[f](b) if f in Smod else Rue(f,b,s+1))+C[a+1:]
				a=C.find("}",c)
				if a is -1:break
	return C
def Smod(x):
	print x
	return ""
Smod={"print":Smod,"input":input,"argv":dict(((str(a),b) for a,b in enumerate(argv[2:]))).__getitem__}
argv="_" if len(argv) is 1 else argv[1]
D=open(argv+".txt").read(9)=="::=debug\n"
#"""
if D:print(Rue(argv))
else:Rue(argv)
"""
from cProfile import run;D=0;run("Rue(argv)")
"""#"""
