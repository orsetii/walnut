# Set architecture to RISC-V for proper disassembly
set architecture riscv

# Convenience function to show surrounding instructions 
define surr_inst
  x/3i $pc-3 
  x/3i $pc + 1
end



# Prevent long disassembly outputs from stopping
set pagination off 

# More disassembly context when stepping instructions
set disassemble-next-line on

# Highlight current execution line during disassembly
set step-mode on

layout split
