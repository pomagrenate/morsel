#!/usr/bin/env python3
"""
Generate matplotlib charts from benchmark CSV data
"""

import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path
import glob
from datetime import datetime

def create_charts_from_csv(csv_path, output_dir):
    """Generate various performance charts from CSV data"""
    
    # Read the CSV data
    df = pd.read_csv(csv_path)
    
    # Create output directory if it doesn't exist
    Path(output_dir).mkdir(parents=True, exist_ok=True)
    
    timestamp = datetime.now().strftime("%Y%m%d-%H%M%S")
    
    # Set style for better looking charts
    plt.style.use('seaborn-v0_8-darkgrid')
    
    # 1. CPU Performance Chart
    fig, axes = plt.subplots(2, 1, figsize=(12, 8))
    
    # CPU Usage breakdown
    cpu_data = df[['CPU_Total_Percent', 'CPU_User_Percent', 'CPU_Privileged_Percent']].iloc[0]
    axes[0].bar(['Total', 'User', 'System'], 
                [cpu_data['CPU_Total_Percent'], cpu_data['CPU_User_Percent'], cpu_data['CPU_Privileged_Percent']],
                color=['#1f77b4', '#2ca02c', '#d62728'])
    axes[0].set_ylabel('CPU Usage (%)')
    axes[0].set_title('CPU Performance Breakdown')
    axes[0].set_ylim(0, 100)
    
    # Add value labels on bars
    for i, v in enumerate([cpu_data['CPU_Total_Percent'], cpu_data['CPU_User_Percent'], cpu_data['CPU_Privileged_Percent']]):
        axes[0].text(i, v + 1, f'{v:.1f}%', ha='center', va='bottom')
    
    # Processor Queue Length
    axes[1].bar(['Queue Length'], [df['ProcessorQueueLength'].iloc[0]], 
                color='#ff7f0e', alpha=0.7)
    axes[1].set_ylabel('Queue Length')
    axes[1].set_title('Processor Queue Length')
    
    plt.tight_layout()
    cpu_chart_path = f"{output_dir}/cpu_performance_{timestamp}.png"
    plt.savefig(cpu_chart_path, dpi=150, bbox_inches='tight')
    plt.close()
    print(f"CPU chart saved: {cpu_chart_path}")
    
    # 2. Memory Performance Chart
    fig, axes = plt.subplots(2, 2, figsize=(14, 10))
    
    # Available Memory
    mem_available = df['Memory_Available_MB'].iloc[0]
    axes[0, 0].bar(['Available Memory'], [mem_available], color='#2ca02c', alpha=0.7)
    axes[0, 0].set_ylabel('Memory (MB)')
    axes[0, 0].set_title('Available Memory')
    axes[0, 0].text(0, mem_available + 10, f'{mem_available:.0f} MB', ha='center', va='bottom')
    
    # Committed Memory
    mem_committed = df['Memory_Committed_Bytes'].iloc[0] / (1024**3)  # Convert to GB
    axes[0, 1].bar(['Committed Memory'], [mem_committed], color='#d62728', alpha=0.7)
    axes[0, 1].set_ylabel('Memory (GB)')
    axes[0, 1].set_title('Committed Memory')
    axes[0, 1].text(0, mem_committed + 0.5, f'{mem_committed:.2f} GB', ha='center', va='bottom')
    
    # Pages/sec
    pages_sec = df['Memory_Pages_PerSec'].iloc[0]
    axes[1, 0].bar(['Pages/sec'], [pages_sec], color='#9467bd', alpha=0.7)
    axes[1, 0].set_ylabel('Pages/sec')
    axes[1, 0].set_title('Memory Pages/sec')
    axes[1, 0].text(0, pages_sec + 5, f'{pages_sec:.0f}', ha='center', va='bottom')
    
    # Page Faults/sec
    page_faults = df['Memory_PageFaults_PerSec'].iloc[0]
    axes[1, 1].bar(['Page Faults/sec'], [page_faults], color='#8c564b', alpha=0.7)
    axes[1, 1].set_ylabel('Page Faults/sec')
    axes[1, 1].set_title('Page Faults/sec')
    axes[1, 1].text(0, page_faults + 1000, f'{page_faults:.0f}', ha='center', va='bottom')
    
    plt.suptitle('Memory Performance Metrics', fontsize=16, y=1.02)
    plt.tight_layout()
    memory_chart_path = f"{output_dir}/memory_performance_{timestamp}.png"
    plt.savefig(memory_chart_path, dpi=150, bbox_inches='tight')
    plt.close()
    print(f"Memory chart saved: {memory_chart_path}")
    
    # 3. Disk Performance Chart
    fig, axes = plt.subplots(2, 2, figsize=(14, 10))
    
    # Disk Time
    disk_time = df['Disk_Time_Percent'].iloc[0]
    axes[0, 0].bar(['Disk Time'], [disk_time], color='#ff7f0e', alpha=0.7)
    axes[0, 0].set_ylabel('Disk Time (%)')
    axes[0, 0].set_title('Disk Utilization')
    axes[0, 0].set_ylim(0, 100)
    axes[0, 0].text(0, disk_time + 2, f'{disk_time:.1f}%', ha='center', va='bottom')
    
    # Disk Latency
    read_latency = df['Disk_Read_Latency_ms'].iloc[0]
    write_latency = df['Disk_Write_Latency_ms'].iloc[0]
    axes[0, 1].bar(['Read Latency', 'Write Latency'], [read_latency, write_latency],
                    color=['#1f77b4', '#2ca02c'], alpha=0.7)
    axes[0, 1].set_ylabel('Latency (ms)')
    axes[0, 1].set_title('Disk Latency')
    for i, v in enumerate([read_latency, write_latency]):
        axes[0, 1].text(i, v + 0.1, f'{v:.2f} ms', ha='center', va='bottom')
    
    # Disk IOPS
    read_iops = df['Disk_Reads_PerSec'].iloc[0]
    write_iops = df['Disk_Writes_PerSec'].iloc[0]
    axes[1, 0].bar(['Read IOPS', 'Write IOPS'], [read_iops, write_iops],
                    color=['#9467bd', '#8c564b'], alpha=0.7)
    axes[1, 0].set_ylabel('IOPS')
    axes[1, 0].set_title('Disk I/O Operations')
    for i, v in enumerate([read_iops, write_iops]):
        axes[1, 0].text(i, v + 10, f'{v:.0f}', ha='center', va='bottom')
    
    # Combined throughput
    total_throughput = read_iops + write_iops
    axes[1, 1].bar(['Total IOPS'], [total_throughput], color='#e377c2', alpha=0.7)
    axes[1, 1].set_ylabel('Total IOPS')
    axes[1, 1].set_title('Combined Disk Throughput')
    axes[1, 1].text(0, total_throughput + 20, f'{total_throughput:.0f}', ha='center', va='bottom')
    
    plt.suptitle('Disk Performance Metrics', fontsize=16, y=1.02)
    plt.tight_layout()
    disk_chart_path = f"{output_dir}/disk_performance_{timestamp}.png"
    plt.savefig(disk_chart_path, dpi=150, bbox_inches='tight')
    plt.close()
    print(f"Disk chart saved: {disk_chart_path}")
    
    # 4. Network Performance Chart
    fig, axes = plt.subplots(1, 2, figsize=(12, 5))
    
    # Network Bytes/sec
    network_bytes = df['Network_Bytes_PerSec'].iloc[0]
    network_kb = network_bytes / 1024
    axes[0].bar(['Network Throughput'], [network_kb], color='#17becf', alpha=0.7)
    axes[0].set_ylabel('Throughput (KB/sec)')
    axes[0].set_title('Network Bytes/sec')
    axes[0].text(0, network_kb + 50, f'{network_kb:.0f} KB/sec', ha='center', va='bottom')
    
    # Network Packets/sec
    network_packets = df['Network_Packets_PerSec'].iloc[0]
    axes[1].bar(['Packet Rate'], [network_packets], color='#bcbd22', alpha=0.7)
    axes[1].set_ylabel('Packets/sec')
    axes[1].set_title('Network Packets/sec')
    axes[1].text(0, network_packets + 1, f'{network_packets:.0f}', ha='center', va='bottom')
    
    plt.suptitle('Network Performance Metrics', fontsize=16, y=1.02)
    plt.tight_layout()
    network_chart_path = f"{output_dir}/network_performance_{timestamp}.png"
    plt.savefig(network_chart_path, dpi=150, bbox_inches='tight')
    plt.close()
    print(f"Network chart saved: {network_chart_path}")
    
    # 5. Comprehensive Dashboard Chart
    fig = plt.figure(figsize=(16, 10))
    gs = fig.add_gridspec(3, 3, hspace=0.3, wspace=0.3)
    
    # CPU section
    ax1 = fig.add_subplot(gs[0, 0])
    ax1.bar(['Total', 'User', 'System'], 
            [cpu_data['CPU_Total_Percent'], cpu_data['CPU_User_Percent'], cpu_data['CPU_Privileged_Percent']],
            color=['#1f77b4', '#2ca02c', '#d62728'])
    ax1.set_ylabel('CPU %')
    ax1.set_title('CPU Usage')
    ax1.set_ylim(0, 100)
    
    # Memory section
    ax2 = fig.add_subplot(gs[0, 1])
    ax2.bar(['Available (MB)', 'Committed (GB)'], 
            [mem_available, mem_committed],
            color=['#2ca02c', '#d62728'])
    ax2.set_title('Memory Usage')
    
    # Disk section
    ax3 = fig.add_subplot(gs[0, 2])
    ax3.bar(['Disk Time %', 'Read IOPS', 'Write IOPS'], 
            [disk_time, read_iops, write_iops],
            color=['#ff7f0e', '#9467bd', '#8c564b'])
    ax3.set_title('Disk Performance')
    
    # Network section
    ax4 = fig.add_subplot(gs[1, 0])
    ax4.bar(['Bytes/sec', 'Packets/sec'], 
            [network_kb, network_packets],
            color=['#17becf', '#bcbd22'])
    ax4.set_title('Network Performance')
    
    # Temperature
    temp = df['Thermal_Temperature_C'].iloc[0]
    ax5 = fig.add_subplot(gs[1, 1])
    ax5.bar(['CPU Temp'], [temp], color='#e377c2', alpha=0.7)
    ax5.set_ylabel('Temperature (°C)')
    ax5.set_title('Thermal Status')
    ax5.text(0, temp + 1, f'{temp:.1f}°C', ha='center', va='bottom')
    
    # Performance summary text
    ax6 = fig.add_subplot(gs[1, 2])
    ax6.axis('off')
    summary_text = f"""
Performance Summary
===================
CPU: {cpu_data['CPU_Total_Percent']:.1f}%
Memory: {mem_available:.0f} MB available
Disk: {disk_time:.1f}% utilization
Network: {network_kb:.0f} KB/sec
Temp: {temp:.1f}°C
    """
    ax6.text(0.1, 0.5, summary_text, fontsize=12, verticalalignment='center',
            bbox=dict(boxstyle='round', facecolor='wheat', alpha=0.3))
    
    # System health indicator
    ax7 = fig.add_subplot(gs[2, :])
    system_health = []
    
    # CPU health (0-100, <80 is good)
    cpu_health = max(0, 100 - cpu_data['CPU_Total_Percent'])
    system_health.append(('CPU', cpu_health))
    
    # Memory health (available > 1GB is good)
    mem_health = min(100, (mem_available / 1024) * 100)
    system_health.append(('Memory', mem_health))
    
    # Disk health (disk time < 50% is good)
    disk_health = max(0, 100 - disk_time * 2)
    system_health.append(('Disk', disk_health))
    
    # Temperature health (< 70°C is good)
    temp_health = max(0, 100 - (temp - 30) * 2)
    system_health.append(('Thermal', temp_health))
    
    health_names = [h[0] for h in system_health]
    health_values = [h[1] for h in system_health]
    health_colors = ['#2ca02c' if v > 70 else '#ff7f0e' if v > 40 else '#d62728' for v in health_values]
    
    ax7.bar(health_names, health_values, color=health_colors, alpha=0.7)
    ax7.set_ylabel('Health Score (%)')
    ax7.set_title('System Health Indicators')
    ax7.set_ylim(0, 100)
    ax7.axhline(y=70, color='green', linestyle='--', alpha=0.5, label='Good threshold')
    ax7.axhline(y=40, color='orange', linestyle='--', alpha=0.5, label='Warning threshold')
    ax7.legend()
    
    plt.suptitle('System Performance Dashboard', fontsize=18, y=0.995)
    dashboard_path = f"{output_dir}/performance_dashboard_{timestamp}.png"
    plt.savefig(dashboard_path, dpi=150, bbox_inches='tight')
    plt.close()
    print(f"Dashboard chart saved: {dashboard_path}")
    
    return {
        'cpu_chart': cpu_chart_path,
        'memory_chart': memory_chart_path,
        'disk_chart': disk_chart_path,
        'network_chart': network_chart_path,
        'dashboard': dashboard_path
    }

def main():
    """Main function to process CSV files and generate charts"""
    
    # Find the latest CSV file
    csv_pattern = "benchmark-results/system_performance_*.csv"
    csv_files = glob.glob(csv_pattern)
    
    if not csv_files:
        print("No CSV files found matching pattern:", csv_pattern)
        return
    
    # Sort by modification time and get the latest
    latest_csv = max(csv_files, key=lambda x: Path(x).stat().st_mtime)
    print(f"Processing: {latest_csv}")
    
    # Output directory
    output_dir = "benchmark-results/charts"
    
    # Generate charts
    try:
        charts = create_charts_from_csv(latest_csv, output_dir)
        print(f"\nAll charts generated successfully in: {output_dir}")
        print(f"Generated files:")
        for chart_type, path in charts.items():
            print(f"   - {chart_type}: {Path(path).name}")
        
        return output_dir
        
    except Exception as e:
        print(f"Error generating charts: {e}")
        return None

if __name__ == "__main__":
    main()