import socket
import json
import time

def enviar_mensagem(source_id, texto, timestamp):
    # O endereço do Nginx configurado no seu docker-compose
    HOST = '127.0.0.1'
    PORT = 8080

    # Monta o dicionário com a exata estrutura do Rust
    msg_dict = {
        "source_id": source_id,
        "payload": texto,
        "timestamp": timestamp,
        "is_concurrent": False
    }

    # Converte para JSON e adiciona a quebra de linha obrigatória (\n)
    msg_json = json.dumps(msg_dict) + "\n"

    try:
        # Cria a conexão TCP
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            print(f"[{source_id}] Conectando ao Nginx ({HOST}:{PORT})...")
            s.connect((HOST, PORT))
            
            print(f"[{source_id}] Enviando: {msg_dict}")
            s.sendall(msg_json.encode('utf-8'))
            
            # Aguarda a resposta do servidor Rust
            data = s.recv(1024)
            print(f"[{source_id}] Resposta do Servidor: {data.decode('utf-8').strip()}\n")
            
    except ConnectionRefusedError:
        print("Erro: Não foi possível conectar. O docker-compose está rodando?")

if __name__ == "__main__":
    print("Iniciando bateria de testes...\n")
    
    # Enviando algumas mensagens para testar o Round Robin do Nginx
    enviar_mensagem("Cliente_Python", "Olá, sistema distribuído!", 0)
    time.sleep(1)
    
    enviar_mensagem("Cliente_Python", "Testando o balanceamento de carga", 0)
    time.sleep(1)
    
    enviar_mensagem("Cliente_Python", "Mensagem 3", 0)