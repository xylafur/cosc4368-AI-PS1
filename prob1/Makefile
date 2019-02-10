FILES=nodes.rs
CC=rustc

OUT=prob1

$(OUT):
	$(CC) $(FILES) -o $(OUT)

remake:
	$(MAKE) clean
	$(MAKE)
	printf "\n\n############# RUNNING\n\n"
	./$(OUT)


clean:
	rm $(OUT)
