# Nome do executável
EXEC = out

# Compilador
CC = gcc

# Flags de compilação
CFLAGS = -Wall -g

# Diretório de arquivos fonte
SRC_DIR = src

# Diretório de objetos compilados
OBJ_DIR = obj

# Diretório de binários
BIN_DIR = bin

# Arquivos fonte
SRCS = $(wildcard $(SRC_DIR)/*.c)

# Arquivos objetos (transforma *.c em *.o)
OBJS = $(SRCS:$(SRC_DIR)/%.c=$(OBJ_DIR)/%.o)

# Regra padrão, compilar e executar
default: $(EXEC)
	@./$(BIN_DIR)/$(EXEC)

# Regra para criar o binário
$(EXEC): $(OBJS) | $(BIN_DIR)
	$(CC) $(CFLAGS) $(OBJS) -o $(BIN_DIR)/$(EXEC)

# Compilar os arquivos objeto
$(OBJ_DIR)/%.o: $(SRC_DIR)/%.c | $(OBJ_DIR)
	$(CC) $(CFLAGS) -c $< -o $@

# Criar o diretório de objetos se não existir
$(OBJ_DIR):
	mkdir -p $(OBJ_DIR)

# Criar o diretório binário se não existir
$(BIN_DIR):
	mkdir -p $(BIN_DIR)

# Limpar arquivos gerados
clean:
	rm -rf $(OBJ_DIR) $(BIN_DIR)

# Limpar apenas os objetos
cleanobj:
	rm -rf $(OBJ_DIR)

# Executar o programa
run: default
