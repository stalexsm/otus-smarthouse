#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from pysocket import SmartClient
import time


if __name__ == "__main__":
    client = SmartClient()
    print(client)

    print("------")
    client.switch_on()
    client.is_enabled()
    print(client.get_state())

    client.get_power()
    print(client.get_state())

    time.sleep(1)

    print("------")

    client.switch_off()
    client.is_enabled()
    print(client.get_state())

    client.get_power()
    print(client.get_state())
