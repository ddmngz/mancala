function place_stone(index){
    console.log("place stone", index);
}

function take_stones(index){
    console.log("take stones", index);
}

function turn_end(){
    console.log("turn end");
}

function game_end(){
    console.log("game end");
}



async function main(){
    const importObject = {
        env:{
            place_stone:place_stone,
            take_stones:take_stones,
            turn_end:turn_end,
            game_end:game_end,
        }
    }
    const results = await WebAssembly.instantiateStreaming(fetch("target/wasm32-unknown-unknown/debug/libmancala.wasm"), importObject);
    const instance = results.instance;
    const functions = instance.exports;



    console.log(functions);

    functions.start_game();
    functions.take_turn(0, 0);


    functions.take_turn(0, 0);

    functions.take_turn(0, 0);

    render(); 

    function render(){
        const p1 = document.getElementById("p1");
        const p2 = document.getElementById("p2");

        const p1_count = functions.p1_count();
        const p2_count = functions.p2_count();
        console.log({p1:p1_count, p2:p2_count});
        set_marbles(p1,functions.p1_count()); 
        set_marbles(p2,functions.p2_count()); 
        const slots = Array.from(document.getElementsByClassName("slot"));
        slots.forEach((slot, index) => {
            set_marbles(slot,functions.marble_count(index+1)); 
        });
    }

    function set_marbles(slot, marble_number){
        let marble = document.createElement("div");
        marble.classList.add("marble");
        slot.replaceChildren(marble);
        for (let i = 0; i< marble_number-1; i++){
            const new_marble = marble.cloneNode()
            slot.appendChild(new_marble);
        }
    }

    function add_marble(slot, marble_number){
    }
}



main()
