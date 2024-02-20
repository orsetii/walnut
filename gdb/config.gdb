# Set architecture to RISC-V for proper disassembly
set architecture riscv

# Convenience function to show surrounding instructions 
define surr_inst
  x/3i $pc-3 
  x/3i $pc + 1
end

# Commands to execute every time the program stops
define hook-stop
  surr_inst  # Show surrounding instructions
  info registers  # Dump all registers
end

# (Optional) Additional useful settings

# Prevent long disassembly outputs from stopping
set pagination off 

# More disassembly context when stepping instructions
set disassemble-next-line on

# Highlight current execution line during disassembly
set step-mode on

layout split
