pub fn get_service_name(port: u16) -> &'static str {
    match port {
        // File Transfer & Remote Access
        20 => "FTP-DATA",
        21 => "FTP",
        22 => "SSH",
        23 => "Telnet",
        69 => "TFTP",
        548 => "AFP",
        873 => "Rsync",
        989 => "FTPS-DATA",
        990 => "FTPS-CTRL",
        2049 => "NFS",
        2121 => "FTP",
        2222 => "SSH-ALT",
        22222 => "SSH-ALT",

        // Email Services
        25 => "SMTP",
        110 => "POP3",
        143 => "IMAP",
        465 => "SMTPS",
        587 => "SMTP",
        993 => "IMAPS",
        995 => "POP3S",

        // Web Services
        80 => "HTTP",
        443 => "HTTPS",
        8000 => "HTTP",
        8008 => "HTTP",
        8080 => "HTTP-Proxy",
        8081 => "HTTP",
        8087 => "HTTP",
        8088 => "HTTP",
        8090 => "HTTP",
        8091 => "HTTP",
        8222 => "NATS",
        8443 => "HTTPS",
        8888 => "HTTP",
        9000 => "HTTP",
        9090 => "HTTP",
        4443 => "HTTPS",

        // DNS & Directory Services
        53 => "DNS",
        389 => "LDAP",
        636 => "LDAPS",

        // Database Services
        1433 => "MSSQL",
        1434 => "MSSQL",
        1521 => "Oracle",
        3306 => "MySQL",
        5432 => "PostgreSQL",
        5984 => "CouchDB",
        6379 => "Redis",
        7000 => "Cassandra-Node",
        7001 => "Cassandra-TLS",
        8086 => "InfluxDB",
        9042 => "Cassandra-CQL",
        9200 => "Elasticsearch",
        9300 => "Elasticsearch",
        11211 => "Memcached",
        27017 => "MongoDB",
        28017 => "MongoDB",
        50000 => "SAP/DB2",

        // Messaging & Queuing
        1883 => "MQTT",
        5672 => "AMQP",
        8883 => "MQTT-SSL",
        9092 => "Kafka",
        15672 => "RabbitMQ-UI",
        61613 => "STOMP",

        // Network Services
        67 => "DHCP-Server",
        68 => "DHCP-Client",
        123 => "NTP",
        161 => "SNMP",
        162 => "SNMP-Trap",
        514 => "Syslog",
        1900 => "SSDP/UPnP",

        // VPN & Tunneling
        500 => "IPSec",
        1194 => "OpenVPN",
        1701 => "L2TP",
        1723 => "PPTP",
        4500 => "IPSec-NAT",

        // Remote Desktop & Management
        3389 => "RDP",
        5900 => "VNC",
        5901 => "VNC",
        5985 => "WinRM",
        5986 => "WinRM-SSL",
        8291 => "MikroTik",
        10000 => "Webmin",

        // Development & CI/CD Tools
        3000 => "Dev-Server",
        3001 => "Dev-Server",
        4200 => "Angular",
        5000 => "Dev-Server",
        5001 => "Dev-Server",
        8983 => "Solr",
        9418 => "Git",

        // Monitoring & Management
        2181 => "Zookeeper",
        8500 => "Consul",
        10050 => "Zabbix",
        10051 => "Zabbix",

        // Hadoop/Big Data
        8020 => "HDFS-RPC",
        50070 => "HDFS-WebUI-2x",

        // Legacy & Other Services
        79 => "Finger",
        111 => "RPC",
        113 => "Ident",
        119 => "NNTP",
        135 => "RPC",
        137 => "NetBIOS",
        138 => "NetBIOS",
        139 => "NetBIOS",
        179 => "BGP",
        194 => "IRC",
        445 => "SMB",
        515 => "LPD",
        631 => "IPP",
        902 => "VMware",
        1080 => "SOCKS",
        2082 => "cPanel",
        2083 => "cPanel-SSL",
        2086 => "WHM",
        2087 => "WHM-SSL",
        3128 => "Proxy",
        3690 => "SVN",
        4444 => "Metasploit-L",
        5060 => "SIP",
        5061 => "SIP-TLS",
        6666 => "IRC",
        6667 => "IRC",
        9001 => "Tor",
        9100 => "Printer",
        9999 => "Dev-Server",

        _ => "Unknown",
    }
}
