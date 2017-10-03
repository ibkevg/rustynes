mod opecode;
mod registers;

use self::opecode::*;
use self::registers::*;
use nes::types::{Data, Addr, Word};

#[derive(Debug)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu { registers: Registers::new() }
    }

    pub fn reset<R>(&mut self, read: R)
        where R: Fn(Addr) -> Data
    {
        self.registers.reset();
        let pc = self.read_word(&read, 0xFFFC);
        self.registers.set_pc(pc);
    }

    pub fn run<R>(&mut self, read: R) -> Data
        where R: Fn(Addr) -> Data
    {
        println!("registers {:?}", self.registers);
        let code = self.fetch(&read);
        let ref map = opecode::MAP;
        let code = &*map.get(&code).unwrap();
        let opeland = self.fetch_opeland(&code, &read);
        match code.name {
            Instruction::LDA => self.lda(&code, opeland, &read),
            Instruction::LDX => self.ldx(&code, opeland, &read),
            Instruction::LDY => self.ldy(&code, opeland, &read),
            Instruction::STA => println!("{}", "TODO:"),
            Instruction::STX => println!("{}", "TODO:"),
            Instruction::STY => println!("{}", "TODO:"),
            Instruction::TXA => println!("{}", "TODO:"),
            Instruction::TYA => println!("{}", "TODO:"),
            Instruction::TXS => println!("{}", "TODO:"),
            Instruction::TAY => println!("{}", "TODO:"),
            Instruction::TAX => println!("{}", "TODO:"),
            Instruction::TSX => println!("{}", "TODO:"),
            Instruction::PHP => println!("{}", "TODO:"),
            Instruction::PLP => println!("{}", "TODO:"),
            Instruction::PHA => println!("{}", "TODO:"),
            Instruction::PLA => println!("{}", "TODO:"),
            Instruction::ADC => println!("{}", "TODO:"),
            Instruction::SBC => println!("{}", "TODO:"),
            Instruction::CPX => println!("{}", "TODO:"),
            Instruction::CPY => println!("{}", "TODO:"),
            Instruction::CMP => println!("{}", "TODO:"),
            Instruction::AND => println!("{}", "TODO:"),
            Instruction::EOR => println!("{}", "TODO:"),
            Instruction::ORA => println!("{}", "TODO:"),
            Instruction::BIT => println!("{}", "TODO:"),
            Instruction::ASL => println!("{}", "TODO:"),
            Instruction::LSR => println!("{}", "TODO:"),
            Instruction::ROL => println!("{}", "TODO:"),
            Instruction::ROR => println!("{}", "TODO:"),
            Instruction::INX => println!("{}", "TODO:"),
            Instruction::INY => println!("{}", "TODO:"),
            Instruction::INC => println!("{}", "TODO:"),
            Instruction::DEX => println!("{}", "TODO:"),
            Instruction::DEY => println!("{}", "TODO:"),
            Instruction::DEC => println!("{}", "TODO:"),
            Instruction::CLC => println!("{}", "TODO:"),
            Instruction::CLI => println!("{}", "TODO:"),
            Instruction::CLV => println!("{}", "TODO:"),
            Instruction::SEC => println!("{}", "TODO:"),
            Instruction::SEI => println!("{}", "TODO:"),
            Instruction::NOP => println!("{}", "TODO:"),
            Instruction::BRK => println!("{}", "TODO:"),
            Instruction::JSR => println!("{}", "TODO:"),
            Instruction::JMP => println!("{}", "TODO:"),
            Instruction::RTI => println!("{}", "TODO:"),
            Instruction::RTS => println!("{}", "TODO:"),
            Instruction::BPL => println!("{}", "TODO:"),
            Instruction::BMI => println!("{}", "TODO:"),
            Instruction::BVC => println!("{}", "TODO:"),
            Instruction::BVS => println!("{}", "TODO:"),
            Instruction::BCC => println!("{}", "TODO:"),
            Instruction::BCS => println!("{}", "TODO:"),
            Instruction::BNE => println!("{}", "TODO:"),
            Instruction::BEQ => println!("{}", "TODO:"),
            Instruction::SED => println!("{}", "TODO:"),
            Instruction::CLD => println!("{}", "TODO:"),
            Instruction::LAX => println!("{}", "TODO:"),
            Instruction::SAX => println!("{}", "TODO:"),
            Instruction::DCP => println!("{}", "TODO:"),
            Instruction::ISB => println!("{}", "TODO:"),
            Instruction::SLO => println!("{}", "TODO:"),
            Instruction::RLA => println!("{}", "TODO:"),
            Instruction::SRE => println!("{}", "TODO:"),
            Instruction::RRA => println!("{}", "TODO:"),
        }
        code.cycle
    }

    fn fetch<R>(&mut self, read: R) -> Data
        where R: Fn(Addr) -> Data
    {
        let code = read(self.registers.get_pc());
        self.registers.update_pc();
        code
    }

    fn read_word<R>(&self, read: R, addr: Addr) -> Word
        where R: Fn(Addr) -> Data
    {
        let low = read(addr) as Word;
        let high = read(addr + 1) as Word;
        (high << 8 | low) as Word
    }

    fn fetch_opeland<F>(&mut self, code: &Opecode, read: F) -> Word
        where F: Fn(Addr) -> Data
    {
        match code.mode {
            Addressing::Accumulator => 0x0000,
            Addressing::Implied => 0x0000,
            Addressing::Immediate => self.fetch(read) as Word,
            Addressing::Relative => {
                let base = self.fetch(read) as Word;
                if base < 0x80 {
                    base + self.registers.get_pc()
                } else {
                    base + self.registers.get_pc() - 256
                }
            }
            Addressing::ZeroPage => self.fetch(read) as Word,
            Addressing::ZeroPageX => {
                let addr = self.fetch(read) as Word;
                (addr + self.registers.get(ByteRegister::X) as Word) & 0xFF as Word
            }
            _ => 10u16,
        }
        /*

      case 'zeroPageX': {
        const addr = this.fetch(this.registers.PC);
        return {
          addrOrData: (addr + this.registers.X) & 0xFF,
          additionalCycle: 0,
        }
      }
      case 'zeroPageY': {
        const addr = this.fetch(this.registers.PC);
        return {
          addrOrData: (addr + this.registers.Y & 0xFF),
          additionalCycle: 0,
        }
      }
      case 'absolute': {
        return {
          addrOrData: (this.fetch(this.registers.PC, "Word")),
          additionalCycle: 0,
        }
      }
      case 'absoluteX': {
        const addr = (this.fetch(this.registers.PC, "Word"));
        const additionalCycle = (addr & 0xFF00) !== ((addr + this.registers.X) & 0xFF00) ? 1 : 0;
        return {
          addrOrData: (addr + this.registers.X) & 0xFFFF,
          additionalCycle,
        }
      }
      case 'absoluteY': {
        const addr = (this.fetch(this.registers.PC, "Word"));
        const additionalCycle = (addr & 0xFF00) !== ((addr + this.registers.Y) & 0xFF00) ? 1 : 0;
        return {
          addrOrData: (addr + this.registers.Y) & 0xFFFF,
          additionalCycle,
        }
      }
      case 'preIndexedIndirect': {
        const baseAddr = (this.fetch(this.registers.PC) + this.registers.X) & 0xFF;
        const addr = this.read(baseAddr) + (this.read((baseAddr + 1) & 0xFF) << 8);
        return {
          addrOrData: addr & 0xFFFF,
          additionalCycle: (addr & 0xFF00) !== (baseAddr & 0xFF00) ? 1 : 0,
        }
      }
      case 'postIndexedIndirect': {
        const addrOrData = this.fetch(this.registers.PC);
        const baseAddr = this.read(addrOrData) + (this.read((addrOrData + 1) & 0xFF) << 8);
        const addr = baseAddr + this.registers.Y;
        return {
          addrOrData: addr & 0xFFFF,
          additionalCycle: (addr & 0xFF00) !== (baseAddr & 0xFF00) ? 1 : 0,
        }
      }
      case 'indirectAbsolute': {
        const addrOrData = this.fetch(this.registers.PC, "Word");
        const addr = this.read(addrOrData) + (this.read((addrOrData & 0xFF00) | (((addrOrData & 0xFF) + 1) & 0xFF)) << 8);
        return {
          addrOrData: addr & 0xFFFF,
          additionalCycle: 0,
        }
      }*/
        // this.registers.A = if code.mode === Addressing::Immediate {
        //     addrOrData
        // } else {
        //     this.read(addrOrData)
        // }
        // this.registers.P.negative = !!(this.registers.A & 0x80);
        // this.registers.P.zero = !this.registers.A;
    }

    fn lda<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .set_acc(computed)
            .update_negative(computed)
            .update_zero(computed);
    }

    fn ldx<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .set_x(computed)
            .update_negative(computed)
            .update_zero(computed);
    }

    fn ldy<R>(&mut self, code: &Opecode, opeland: Word, read: R)
        where R: Fn(Addr) -> Data
    {
        let computed = match code.mode {
            Addressing::Immediate => opeland as Data,
            _ => read(opeland),
        };
        self.registers
            .set_y(computed)
            .update_negative(computed)
            .update_zero(computed);
    }
    /*

      case 'STA': {
        this.write(addrOrData, this.registers.A);
        break;
      }
      case 'STX': {
        this.write(addrOrData, this.registers.X);
        break;
      }
      case 'STY': {
        this.write(addrOrData, this.registers.Y);
        break;
      }
      case 'TAX': {
        this.registers.X = this.registers.A;
        this.registers.P.negative = !!(this.registers.X & 0x80);
        this.registers.P.zero = !this.registers.X;
        break;
      }
      case 'TAY': {
        this.registers.Y = this.registers.A;
        this.registers.P.negative = !!(this.registers.Y & 0x80);
        this.registers.P.zero = !this.registers.Y;
        break;
      }
      case 'TSX': {
        this.registers.X = this.registers.SP & 0xFF;
        this.registers.P.negative = !!(this.registers.X & 0x80);
        this.registers.P.zero = !this.registers.X;
        break;
      }
      case 'TXA': {
        this.registers.A = this.registers.X;
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !this.registers.A;
        break;
      }
      case 'TXS': {
        this.registers.SP = this.registers.X + 0x0100;
        break;
      }
      case 'TYA': {
        this.registers.A = this.registers.Y;
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !this.registers.A;
        break;
      }
      case 'ADC': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const operated = data + this.registers.A + this.registers.P.carry;
        const overflow = (!(((this.registers.A ^ data) & 0x80) != 0) && (((this.registers.A ^ operated) & 0x80)) != 0);
        this.registers.P.overflow = overflow;
        this.registers.P.carry = operated > 0xFF;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !(operated & 0xFF);
        this.registers.A = operated & 0xFF;
        break;
      }
      case 'AND': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const operated = data & this.registers.A;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !operated;
        this.registers.A = operated & 0xFF;
        break;
      }
      case 'ASL': {
        if (mode === 'accumulator') {
          const acc = this.registers.A;
          this.registers.P.carry = !!(acc & 0x80);
          this.registers.A = (acc << 1) & 0xFF;
          this.registers.P.zero = !this.registers.A;
          this.registers.P.negative = !!(this.registers.A & 0x80);
        } else {
          const data = this.read(addrOrData);
          this.registers.P.carry = !!(data & 0x80);
          const shifted = (data << 1) & 0xFF;
          this.write(addrOrData, shifted);
          this.registers.P.zero = !shifted;
          this.registers.P.negative = !!(shifted & 0x80);
        }
        break;
      }
      case 'BIT': {
        const data = this.read(addrOrData);
        this.registers.P.negative = !!(data & 0x80);
        this.registers.P.overflow = !!(data & 0x40);
        this.registers.P.zero = !(this.registers.A & data);
        break;
      }
      case 'CMP': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const compared = this.registers.A - data;
        this.registers.P.carry = compared >= 0;
        this.registers.P.negative = !!(compared & 0x80);
        this.registers.P.zero = !(compared & 0xff);
        break;
      }
      case 'CPX': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const compared = this.registers.X - data;
        this.registers.P.carry = compared >= 0;
        this.registers.P.negative = !!(compared & 0x80);
        this.registers.P.zero = !(compared & 0xff);
        break;
      }
      case 'CPY': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const compared = this.registers.Y - data;
        this.registers.P.carry = compared >= 0;
        this.registers.P.negative = !!(compared & 0x80);
        this.registers.P.zero = !(compared & 0xff);
        break;
      }
      case 'DEC': {
        const data = (this.read(addrOrData) - 1) & 0xFF;
        this.registers.P.negative = !!(data & 0x80);
        this.registers.P.zero = !data;
        this.write(addrOrData, data);
        break;
      }
      case 'DEX': {
        this.registers.X = (this.registers.X - 1) & 0xFF;
        this.registers.P.negative = !!(this.registers.X & 0x80);
        this.registers.P.zero = !this.registers.X;
        break;
      }
      case 'DEY': {
        this.registers.Y = (this.registers.Y - 1) & 0xFF;
        this.registers.P.negative = !!(this.registers.Y & 0x80);
        this.registers.P.zero = !this.registers.Y;
        break;
      }
      case 'EOR': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const operated = data ^ this.registers.A;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !operated;
        this.registers.A = operated & 0xFF;
        break;
      }
      case 'INC': {
        const data = (this.read(addrOrData) + 1) & 0xFF;
        this.registers.P.negative = !!(data & 0x80);
        this.registers.P.zero = !data;
        this.write(addrOrData, data);
        break;
      }
      case 'INX': {
        this.registers.X = (this.registers.X + 1) & 0xFF;
        this.registers.P.negative = !!(this.registers.X & 0x80);
        this.registers.P.zero = !this.registers.X;
        break;
      }
      case 'INY': {
        this.registers.Y = (this.registers.Y + 1) & 0xFF;
        this.registers.P.negative = !!(this.registers.Y & 0x80);
        this.registers.P.zero = !this.registers.Y;
        break;
      }
      case 'LSR': {
        if (mode === 'accumulator') {
          const acc = this.registers.A & 0xFF;
          this.registers.P.carry = !!(acc & 0x01);
          this.registers.A = acc >> 1;
          this.registers.P.zero = !this.registers.A;
        } else {
          const data = this.read(addrOrData);
          this.registers.P.carry = !!(data & 0x01);
          this.registers.P.zero = !(data >> 1);
          this.write(addrOrData, data >> 1);
        }
        this.registers.P.negative = false;
        break;
      }
      case 'ORA': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const operated = data | this.registers.A;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !operated;
        this.registers.A = operated & 0xFF;
        break;
      }
      case 'ROL': {
        if (mode === 'accumulator') {
          const acc = this.registers.A;
          this.registers.A = (acc << 1) & 0xFF | (this.registers.P.carry ? 0x01 : 0x00);
          this.registers.P.carry = !!(acc & 0x80);
          this.registers.P.zero = !this.registers.A;
          this.registers.P.negative = !!(this.registers.A & 0x80);
        } else {
          const data = this.read(addrOrData);
          const writeData = (data << 1 | (this.registers.P.carry ? 0x01 : 0x00)) & 0xFF;
          this.write(addrOrData, writeData);
          this.registers.P.carry = !!(data & 0x80);
          this.registers.P.zero = !writeData;
          this.registers.P.negative = !!(writeData & 0x80);
        }
        break;
      }
      case 'ROR': {
        if (mode === 'accumulator') {
          const acc = this.registers.A;
          this.registers.A = acc >> 1 | (this.registers.P.carry ? 0x80 : 0x00);
          this.registers.P.carry = !!(acc & 0x01);
          this.registers.P.zero = !this.registers.A;
          this.registers.P.negative = !!(this.registers.A & 0x80);
        } else {
          const data = this.read(addrOrData);
          const writeData = data >> 1 | (this.registers.P.carry ? 0x80 : 0x00);
          this.write(addrOrData, writeData);
          this.registers.P.carry = !!(data & 0x01);
          this.registers.P.zero = !writeData;
          this.registers.P.negative = !!(writeData & 0x80);
        }
        break;
      }
      case 'SBC': {
        const data = mode === 'immediate' ? addrOrData : this.read(addrOrData);
        const operated = this.registers.A - data - (this.registers.P.carry ? 0 : 1);
        const overflow = (((this.registers.A ^ operated) & 0x80) != 0 && ((this.registers.A ^ data) & 0x80) != 0);
        this.registers.P.overflow = overflow;
        this.registers.P.carry = operated >= 0;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !(operated & 0xFF);
        this.registers.A = operated & 0xFF;
        break;
      }
      case 'PHA': {
        this.push(this.registers.A);
        break;
      }
      case 'PHP': {
        this.registers.P.break = true;
        this.pushStatus();
        break;
      }
      case 'PLA': {
        this.registers.A = this.pop();
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !this.registers.A;
        break;
      }
      case 'PLP': {
        this.popStatus();
        this.registers.P.reserved = true;
        break;
      }
      case 'JMP': {
        this.registers.PC = addrOrData;
        break;
      }
      case 'JSR': {
        const PC = this.registers.PC - 1;
        this.push((PC >> 8) & 0xFF);
        this.push(PC & 0xFF);
        this.registers.PC = addrOrData;
        break;
      }
      case 'RTS': {
        this.popPC();
        this.registers.PC++;
        break;
      }
      case 'RTI': {
        this.popStatus();
        this.popPC();
        this.registers.P.reserved = true;
        break;
      }
      case 'BCC': {
        if (!this.registers.P.carry) this.branch(addrOrData);
        break;
      }
      case 'BCS': {
        if (this.registers.P.carry) this.branch(addrOrData);
        break;
      }
      case 'BEQ': {
        if (this.registers.P.zero) this.branch(addrOrData);
        break;
      }
      case 'BMI': {
        if (this.registers.P.negative) this.branch(addrOrData);
        break;
      }
      case 'BNE': {
        if (!this.registers.P.zero) this.branch(addrOrData);
        break;
      }
      case 'BPL': {
        if (!this.registers.P.negative) this.branch(addrOrData);
        break;
      }
      case 'BVS': {
        if (this.registers.P.overflow) this.branch(addrOrData);
        break;
      }
      case 'BVC': {
        if (!this.registers.P.overflow) this.branch(addrOrData);
        break;
      }
      case 'CLD': {
        this.registers.P.decimal = false;
        break;
      }
      case 'CLC': {
        this.registers.P.carry = false;
        break;
      }
      case 'CLI': {
        this.registers.P.interrupt = false;
        break;
      }
      case 'CLV': {
        this.registers.P.overflow = false;
        break;
      }
      case 'SEC': {
        this.registers.P.carry = true;
        break;
      }
      case 'SEI': {
        this.registers.P.interrupt = true;
        break;
      }
      case 'SED': {
        this.registers.P.decimal = true;
        break;
      }
      case 'BRK': {
        const interrupt = this.registers.P.interrupt;
        this.registers.PC++;
        this.push((this.registers.PC >> 8) & 0xFF);
        this.push(this.registers.PC & 0xFF);
        this.registers.P.break = true;
        this.pushStatus();
        this.registers.P.interrupt = true;
        // Ignore interrupt when already set.
        if (!interrupt) {
          this.registers.PC = this.read(0xFFFE, "Word");
        }
        this.registers.PC--;
        break;
      }
      case 'NOP': {
        break;
      }
      // Unofficial Opecode
      case 'NOPD': {
        this.registers.PC++;
        break;
      }
      case 'NOPI': {
        this.registers.PC += 2;
        break;
      }
      case 'LAX': {
        this.registers.A = this.registers.X = this.read(addrOrData);
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !this.registers.A;
        break;
      }
      case 'SAX': {
        const operated = this.registers.A & this.registers.X;
        this.write(addrOrData, operated);
        break;
      }
      case 'DCP': {
        const operated = (this.read(addrOrData) - 1) & 0xFF;
        this.registers.P.negative = !!(((this.registers.A - operated) & 0x1FF) & 0x80);
        this.registers.P.zero = !((this.registers.A - operated) & 0x1FF);
        this.write(addrOrData, operated);
        break;
      }
      case 'ISB': {
        const data = (this.read(addrOrData) + 1) & 0xFF;
        const operated = (~data & 0xFF) + this.registers.A + this.registers.P.carry;
        const overflow = (!(((this.registers.A ^ data) & 0x80) != 0) && (((this.registers.A ^ operated) & 0x80)) != 0);
        this.registers.P.overflow = overflow;
        this.registers.P.carry = operated > 0xFF;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !(operated & 0xFF);
        this.registers.A = operated & 0xFF;
        this.write(addrOrData, data);
        break;
      }
      case 'SLO': {
        let data = this.read(addrOrData);
        this.registers.P.carry = !!(data & 0x80);
        data = (data << 1) & 0xFF;
        this.registers.A |= data;
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !(this.registers.A & 0xFF);
        this.write(addrOrData, data);
        break;
      }
      case 'RLA': {
        const data = (this.read(addrOrData) << 1) + this.registers.P.carry;
        this.registers.P.carry = !!(data & 0x100);
        this.registers.A = (data & this.registers.A) & 0xFF;
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !(this.registers.A & 0xFF);
        this.write(addrOrData, data);
        break;
      }
      case 'SRE': {
        let data = this.read(addrOrData);
        this.registers.P.carry = !!(data & 0x01)
        data >>= 1;
        this.registers.A ^= data;
        this.registers.P.negative = !!(this.registers.A & 0x80);
        this.registers.P.zero = !(this.registers.A & 0xFF);
        this.write(addrOrData, data);
        break;
      }
      case 'RRA': {
        let data = this.read(addrOrData);
        const carry = !!(data & 0x01);
        data = (data >> 1) | (this.registers.P.carry ? 0x80 : 0x00);
        const operated = data + this.registers.A + carry;
        const overflow = (!(((this.registers.A ^ data) & 0x80) != 0) && (((this.registers.A ^ operated) & 0x80)) != 0);
        this.registers.P.overflow = overflow;
        this.registers.P.negative = !!(operated & 0x80);
        this.registers.P.zero = !(operated & 0xFF);
        this.registers.A = operated & 0xFF;
        this.registers.P.carry = operated > 0xFF;
        this.write(addrOrData, data);
        break;
      }
      */
}


#[test]
fn lda_immidiate() {
    let mut cpu = Cpu::new();
    cpu.registers.PC = 0x0000;
    let rom = vec![0x00];
    let code = Opecode {
        name: Instruction::LDA,
        mode: Addressing::Immediate,
        cycle: 1,
    };
    cpu.lda(&code, 255, |addr: Addr| rom[addr as usize]);
    assert!(cpu.registers.A == 255);
}

#[test]
fn ldx_immidiate() {
    let mut cpu = Cpu::new();
    cpu.registers.PC = 0x0000;
    let rom = vec![0x00];
    let code = Opecode {
        name: Instruction::LDX,
        mode: Addressing::Immediate,
        cycle: 1,
    };
    cpu.ldx(&code, 255, |addr: Addr| rom[addr as usize]);
    assert!(cpu.registers.X == 255);
}