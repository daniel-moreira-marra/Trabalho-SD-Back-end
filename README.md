# Trabalho SD - Back-end

Este é o back-end para um projeto de sistemas distribuídos, um chat que utiliza os Clocks de Lamport para garantir a ordem das mensagens.

## Infraestrutura

A infraestrutura é orquestrada com o Docker Compose e consiste em:

- **3 instâncias do back-end:** Atualmente, são placeholders que ecoam qualquer tráfego TCP que recebem. Futuramente, serão substituídos pela aplicação em Rust.
- **1 Load Balancer:** Um Nginx configurado para fazer o balanceamento de carga TCP para as 3 instâncias do back-end.

## Como testar

Para testar a infraestrutura atual, você pode seguir os seguintes passos:

1.  **Subir os contêineres:**

    ```bash
    docker-compose up -d
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

    Qualquer mensagem que você enviar será ecoada de volta por uma das instâncias do back-end.
