
function genCreateIBCInstructionData(
    initStateData: Uint8Array,
): Uint8Array {
    const innerLength = initStateData.length + 1;
    const fullLength = innerLength + 4;

    const instructionData = new Uint8Array(fullLength);
    let offset = 0;
    
    const dv = new DataView(instructionData.buffer);

    dv.setUint32(offset, innerLength, true);
    offset += 4;


    // instruction id -> 0 == create
    instructionData[offset] = 0;
    offset += 1;


    instructionData.set(initStateData, offset);
    offset += initStateData.length;

    return instructionData;
}


function genAppendBlockIBCInstructionData(
    blockNumber: number,
    blockHeaderBytes: Uint8Array,
): Uint8Array {
    const innerLength = blockHeaderBytes.length + 1 + 4 + 1;
    const fullLength = innerLength + 4;

    const instructionData = new Uint8Array(fullLength);
    let offset = 0;
    
    const dv = new DataView(instructionData.buffer);

    dv.setUint32(offset, innerLength, true);
    offset += 4;


    // instruction id -> 1 == append
    instructionData[offset] = 1;
    offset += 1;


    dv.setUint32(offset, blockNumber, true);
    offset += 4;


    // mode 0 == non-zkp mode
    instructionData[offset] = 0;
    offset += 1;


    instructionData.set(blockHeaderBytes, offset);
    offset += blockHeaderBytes.length;

    return instructionData;
}


function genAppendBlockZKPIBCInstructionData(
    blockNumber: number,
    blockHeaderBytes: Uint8Array,
    scryptHash: Uint8Array,
    proof: Uint8Array,
): Uint8Array {
    if(scryptHash.length != 32){
        throw new Error("scryptHash must be 32 bytes long");
    }else if(proof.length != 260){
        throw new Error("proof must be 260 bytes long");
    }

    const innerLength = blockHeaderBytes.length + 1 + 4 + 1 + 32 + 260;
    const fullLength = innerLength + 4;

    const instructionData = new Uint8Array(fullLength);
    let offset = 0;
    
    const dv = new DataView(instructionData.buffer);

    dv.setUint32(offset, innerLength, true);
    offset += 4;


    // instruction id -> 1 == append
    instructionData[offset] = 1;
    offset += 1;


    dv.setUint32(offset, blockNumber, true);
    offset += 4;


    // mode 1 == zkp mode
    instructionData[offset] = 1;
    offset += 1;

    instructionData.set(scryptHash, offset);
    offset += scryptHash.length;


    instructionData.set(blockHeaderBytes, offset);
    offset += blockHeaderBytes.length;

    
    instructionData.set(proof, offset);
    offset += proof.length;

    return instructionData;
}


function genAppendBlockZKPIBCInstructionDataStart(
    blockNumber: number,
    blockHeaderBytes: Uint8Array,
    scryptHash: Uint8Array,
): Uint8Array {
    if(scryptHash.length != 32){
        throw new Error("scryptHash must be 32 bytes long");
    }

    const innerLength = blockHeaderBytes.length + 1 + 4 + 1 + 32;
    const fullLength = innerLength + 4;

    const instructionData = new Uint8Array(fullLength);
    let offset = 0;
    
    const dv = new DataView(instructionData.buffer);

    dv.setUint32(offset, innerLength, true);
    offset += 4;


    // instruction id -> 1 == append
    instructionData[offset] = 1;
    offset += 1;


    dv.setUint32(offset, blockNumber, true);
    offset += 4;


    // mode 1 == zkp mode
    instructionData[offset] = 1;
    offset += 1;

    instructionData.set(scryptHash, offset);
    offset += scryptHash.length;


    instructionData.set(blockHeaderBytes, offset);
    offset += blockHeaderBytes.length;

    return instructionData;
}

export {
    genCreateIBCInstructionData,
    genAppendBlockIBCInstructionData,
    genAppendBlockZKPIBCInstructionData,
    genAppendBlockZKPIBCInstructionDataStart,
}