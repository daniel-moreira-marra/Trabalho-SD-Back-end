import socket
import json
import threading

def enviar_mensagem_simultanea(source_id, texto):
    HOST = '127.0.0.1'
    PORT = 8080
    
    msg_dict = {
        "source_id": source_id,
        "payload": texto,
        "timestamp": 0,
        "is_concurrent": False
    }
    
    msg_json = json.dumps(msg_dict) + "\n"
    
    try:
        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.connect((HOST, PORT))
            s.sendall(msg_json.encode('utf-8'))
            data = s.recv(1024)
            print(f"[{source_id}] Resposta: {data.decode('utf-8').strip()}")
    except ConnectionRefusedError:
        print(f"Erro: Não foi possível conectar o {source_id}.")

if __name__ == "__main__":
    print("Iniciando ataque simultâneo ao Load Balancer...\n")
    
    # Criando duas threads para enviar mensagens
    thread1 = threading.Thread(target=enviar_mensagem_simultanea, args=("Cliente_A", "Mensagem Concorrente 1"))
    thread2 = threading.Thread(target=enviar_mensagem_simultanea, args=("Cliente_B", "Mensagem Concorrente 2"))
    
    thread1.start()
    thread2.start()
    
    # Aguarda as duas terminarem
    thread1.join()
    thread2.join()
    
    print("\nTeste de concorrência finalizado. Verifique os logs do Docker!")