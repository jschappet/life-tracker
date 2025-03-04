#!/usr/bin/bash

./life-tracker  > life-tracker.log 2>&1 &
echo $$ > pid 