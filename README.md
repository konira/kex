# kex
konira exfiltration 

# Projeto de Exfiltração de Dados

Este projeto é uma implementação de um sistema de exfiltração de dados e commands. Ele utiliza o protocolo ICMP para a transmissão de dados e o padrão Observable (EventEmitter) para a gestão de eventos.

## Dependências

O projeto depende dos seguintes pacotes:

- `etherparse`: Uma biblioteca para análise e construção de pacotes Ethernet, IP e TCP. A versão utilizada é a 0.15.

- `pnet`: Uma biblioteca de baixo nível para a manipulação de pacotes de rede. A versão utilizada é a 0.35.

- `Packet.lib` : Lib c++.  A versão utilizada é a 4.1.2

- `wpcap.lib`: Lib c++. A versão utilizada é a 4.1.2
## Como usar

Para usar este projeto, você precisará instalar as dependências listadas acima. Uma vez instaladas, você pode executar o projeto a partir da linha de comando.
passando a "interface net" e uma assinatura de 14 bytes para que o pacote icmp seja reconhecido como um comando ex:

 ```powershell
    kex_app "\Device\NPF_{6C21106D-6B9C-40A5-9800-96CABC3B935D}" "abcd&¨*()09876"
 ```


## Proximos passos

- exportação geração de DLL para injecção.

## Contribuindo

Contribuições para este projeto são bem-vindas. Por favor, abra um problema ou uma solicitação de pull request para contribuir.

## Licença

Este projeto está licenciado sob os termos da licença MIT.
