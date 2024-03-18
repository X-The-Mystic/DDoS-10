# DDoS-10
This is a minor improvement upon the previous attack code, being able to target multiple IPv4 and IPv6 addresses, along with website domain names. I also edited the code to be compatible with botnets, and resolved the errors with the HTTP, SYN, and UDP flood attack vectors.

# How to use:
Open a terminal and type
`git clone https://github.com/X-The-Mystic/DDoS-10`.
Then, type 
`cd DDoS-10`.
Finally, to run the code, type 
`python3 main.py`.

# Some Tips:
Before launching the attack, use the command 
`nmap -Pn (port no.) (IPv4 address)`.
While attack is in progress, use a tool such as Wireshark to view the packets, and to see the impact on your computer, use a tool such as Task Manager.

This code was made for educational and pentesting purposes only. I am not responsible for any illegal or irresponsible uses of this code.
