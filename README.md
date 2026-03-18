# Trabalho-SD-Back-end

Este é o back-end para um projeto de sistemas distribuídos, um chat que utiliza os Clocks de Lamport para garantir a ordem das mensagens.

## Requisitos

Para compilar, executar e testar o projeto, você precisará ter instalado em sua máquina:
- **Docker e Docker Compose** (Docker Desktop ou OrbStack) para a orquestração dos contêineres.
- **Python 3.x** para a execução dos scripts automatizados de teste de rede.
- Ferramentas de terminal `telnet` ou `nc` (Netcat) para testes manuais TCP.

## Infraestrutura

A infraestrutura é orquestrada com o Docker Compose e consiste em:

- **1 Load Balancer (Front-end):** Um Nginx configurado com o módulo `stream` atuando como gateway e realizando o balanceamento de carga TCP (Round Robin) na porta `8080`.
- **3 Instâncias de Back-end (Rust):** Nós distribuídos rodando independentemente que recebem as conexões, atualizam seus Relógios de Lamport, realizam *broadcast* interno para os nós vizinhos (garantindo consistência) e aplicam algoritmos de ordenação parcial e total.

## Como testar

Para testar a infraestrutura atual, você pode seguir os seguintes passos:

1.  **Criar e subir os contêineres:**
    Execute na pasta raiz do projeto (originalmente com o nome Trabalho-SD-Back-end) e certifique-se de já ter inicializado a aplicação de orquestração de contâineres (Docker Desktop ou Orbstack).
    ```bash
    docker-compose up --build
    ```

2.  **Testar a conexão:**

    Você pode usar `telnet` ou `nc` para se conectar ao load balancer na porta `8080`.

    ```bash
    telnet localhost 8080
    ```

    ou

    ```bash
    nc localhost 8080
    ```

    Qualquer mensagem que você enviar será ecoada de volta por uma das instâncias do back-end com a subsequente atualização dos valores dos relógios lógicos de lamport. Exemplo: após iniciar a aplicação (subir os contêineres) abra um novo terminal e digite nc localhost 8080 na pasta raiz do projeto. O terminal vai aguardar a mensagem JSON então digite {"source_id":"Cliente_A", "payload":"Oi!", "timestamp":0, "is_concurrent":false} e pressione ENTER (caractere interpretado como de \n) e o reultado deve ser {"source_id":"Cliente_A","payload":"Oi!","timestamp":1,"is_concurrent":false,"is_broadcast":false,"forwarder_id":null}. Repare que a variável timestamp incrementou, então faça isso novamente e observe {"source_id":"Cliente_A","payload":"Oi!","timestamp":2,"is_concurrent":false,"is_broadcast":false,"forwarder_id":null}, timestamp agora é 2.

3.  **Testar o sistema distribuído:**

    Após subir a aplicação no passo 1, pegue como exemplo de testes os programas em Python juntos à raiz do projeto no repositório. Observe que são 2 e tratam de 2 problemas distintos, sendo o primeiro (cliente_teste.py) para aboradar a sincronização dos relógios em broadcasting (comunicação entre os nós) e o segundo (teste_concorrencia.py) para abordar a questão da concorrência com threads nesse sistema distribuído em que é usado o identificador do nó NODE_ID como critério de desempate. Para testar, deixe a aplicação em execução e execute os programas Python mencionados (1 por vez) e observe os resultados na saída do terminal da aplicação, o primeiro vai mostrar a sincronização entre os relógios dos diferentes nós e o segundo vai mostrar o comportamente mediante concorrencia (quando uma colisão ou race condition entre as threads é detectada aparece o indentificador [CONCURRENT] na tela do terminal e o critério de desampate é usado e visualizado pelo usuário).
