CC = gcc
CFLAGS = -O2 
GEN = main.o utility.o inputs.o printgraph.o whatgraph.o randgraph.o \
	geomgraph.o geomcntrl.o verify.o graph.o cheat.o girth.o flat.o \
	weight.o clique.o cycle.o

generator: $(GEN)
	$(CC) $(CFLAGS) -o generator $(GEN) -lm

graph.o: generate.h graph.h makefile

$(GEN) : generate.h makefile

clean: 
	rm *.o