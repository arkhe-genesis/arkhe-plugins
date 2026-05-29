#!/usr/bin/env python3
"""
Hardware Abstraction Layer for FPGA filtering
"""

class FPGAHardwareLayer:
    def __init__(self):
        self.device_ready = False

    def init_device(self):
        self.device_ready = True
        return self.device_ready
