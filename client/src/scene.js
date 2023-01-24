// RIP TECHNOBLADE NEVER FORGET o7

import * as THREE from '../src/html/modules/three.module.js';
import { GLTFLoader } from '../src/html/modules/GLTFLoader.js';
import { OrbitControls } from '../src/html/modules/OrbitControls.js';
import { GUI } from '../src/html/modules/dat.gui.module.js';
import * as functions from '../src/functions/functionsForSite.js';
import {CSS2DRenderer , CSS2DObject} from '../src/html/modules/CSS2DRenderer.js'
import { Interaction } from '../src/html/modules/three.interaction.module'
//import * as THREEx from '/src/html/modules/threex.domevents.js'
let scene, camera, hlight, DataToSend, turtle, loader = new GLTFLoader(), Block = 1, NewBlock, removable_items = [], data;

var req = new XMLHttpRequest();
req.open('GET', document.location, false);
req.send(null);
var TurtleName = req.responseURL
TurtleName = TurtleName.substring(10)
TurtleName = "Zeus"
console.log(TurtleName)

async function init() {

    //let a = document.getElementById('button1')
    //let text = document.createTextNode('minecraftdirt')
    //let buttons = document.getElementsByClassName('first')
    //Array.from(buttons).forEach((button) => {
        //button.innerHTML = "minecraftdiamond_pickaxe"
    //})

    let datochka = await sendData({message: "turtle pos, inventory, direction and blocks", name: TurtleName}, '/turtle/utility')
    let direction = datochka.direction, x = datochka.x, y = datochka.y, z = datochka.z, BlocksArray = datochka.array, inventory = datochka.inventory, SelectedSlot = inventory[16];
    UpdateInventory(inventory, false)
    SetSelectedButton(inventory[16])

    scene = new THREE.Scene();
    scene.background = new THREE.Color(0x616164);
    scene.frustrumCulled = false;
    
    camera = new THREE.PerspectiveCamera(90, window.innerWidth/window.innerHeight,0.01,999999);
    camera.position.set(x + -3, y + 3, z + 3);
    camera.rotation.x = -0.3;

    hlight = new THREE.AmbientLight (0x404040, 6);
    scene.add(hlight);

    const geometry = new THREE.BoxGeometry  ( 1, 1, 1 );

    window.addEventListener('resize', () => {
        const { innerWidth, innerHeight } = window;

        renderer.setSize(innerWidth, innerHeight);
        labelRenderer.setSize(innerWidth, innerHeight)
        camera.aspect = innerWidth / innerHeight;
        camera.updateProjectionMatrix();
    });

    const renderer = new THREE.WebGLRenderer({ antialiasing: true });
    renderer.setSize(window.innerWidth,window.innerHeight);
    document.body.appendChild(renderer.domElement);

    const labelRenderer = new CSS2DRenderer();
    labelRenderer.setSize(window.innerWidth, window.innerHeight);
    labelRenderer.domElement.style.position = 'absolute';
    labelRenderer.domElement.style.top = 0;
    document.body.appendChild(labelRenderer.domElement)


    const interaction = new Interaction(labelRenderer, scene, camera);

    for (var i in BlocksArray) {
        const {BlockSiteX, BlockSiteY, BlockSiteZ, block, color} = BlocksArray[i]
        AddBlockPos(BlockSiteX, BlockSiteY, BlockSiteZ, color, block)
    }

    const controls = new OrbitControls( camera, labelRenderer.domElement );
    controls.update();
    
    function animate() {
        requestAnimationFrame( animate );
        controls.update();
        camera.updateProjectionMatrix();
        labelRenderer.render( scene, camera )
        renderer.render( scene, camera );
    };

    loader.load('/turtle.gltf', function(gltf) {
        turtle = gltf.scene.children[0];
        turtle.position.set(x, y, z);
        controls.target.set(x, y, z);
        if (direction == "south") {
            turtle.rotateZ(THREE.MathUtils.degToRad(180));
        } else if (direction == "east") {
            turtle.rotateZ(THREE.MathUtils.degToRad(90));
        } else if (direction == "west") {
            turtle.rotateZ(THREE.MathUtils.degToRad(-90));
        }
        scene.add(gltf.scene);
        animate(turtle);
    });

    var MovementButton1 = { Forward: async function() {
        let coords = functions.forward(direction, x, z);
                let xx = coords[0], zz = coords[1];
        var data = await MakeArrayAndSend(xx, y, zz, "forward", direction)
        if (data.boolean != false & data.boolean != undefined) {
            coords = functions.forward(direction, x, z);
            x = coords[0], z = coords[1];
            console.log(x, y, z)
            SetPos(turtle,x,y,z)
            AddInspectedBlocks(data,x,y,z)
        }
        //console.log(scene.children[Block].position.z)
        //data = await receiveData()
        //NewBlock = addBlock(Block);
        //removable_items.push(NewBlock)
        //Block = Block + 1
        //console.log(Block)
    }};

    var MovementButton2 = { Left: async function() {
        let Tempdirection = functions.left(direction);
        var data = await MakeArrayAndSend(x, y, z, "left", Tempdirection)
        if (data.boolean != false & data.boolean != undefined) {
            direction = functions.left(direction);
            turtle.rotateZ(THREE.MathUtils.degToRad(-90));
            AddInspectedBlocks(data,x,y,z)
        }
    }};

    var MovementButton3 = { Right: async function() {
        let Tempdirection = functions.right(direction);
        var data = await MakeArrayAndSend(x, y, z, "right", Tempdirection)
        if (data.boolean != false & data.boolean != undefined) {
            direction = functions.right(direction);
            turtle.rotateZ(THREE.MathUtils.degToRad(90));
            AddInspectedBlocks(data,x,y,z)
        }
    }};

    var MovementButton4 = { clear:function() {
        //scene.remove(NewBlock)
        clean()
        //console.log(Block)
        //scene.remove()
    }};

    var MovementButton5 = { Up: async function() {
        let Tosendy = y + 1
            var data = await MakeArrayAndSend(x, Tosendy, z, "up")
            if (data.boolean != false & data.boolean != undefined) {
                y = ++y
                SetPos(turtle,x,y,z)
                AddInspectedBlocks(data,x,y,z)
            }
    }};

    var MovementButton6 = { Down: async function() {
        let Tosendy = y - 1
            var data = await MakeArrayAndSend(x, Tosendy, z, "down")
            if (data.boolean != false & data.boolean != undefined) {
                y = --y
                SetPos(turtle,x,y,z)
                AddInspectedBlocks(data,x,y,z)
            }
    }};
    
    var MovementButton7 = { Back: async function() {
        let coords = functions.back(direction, x, z);
                let xx = coords[0], zz = coords[1];
        var data = await MakeArrayAndSend(xx, y, zz, "back", direction)
        if (data.boolean != false & data.boolean != undefined) {
            coords = functions.back(direction, x, z);
            x = coords[0], z = coords[1];
            console.log(x, y, z)
            SetPos(turtle,x,y,z)
            AddInspectedBlocks(data,x,y,z)
        }
    }}

    let DigUpButton = document.getElementById('digUp'), DigButton = document.getElementById('dig'), DigDownButton = document.getElementById('digDown'), DropUp = document.getElementById('dropUp'), Drop = document.getElementById('drop'), DropDown = document.getElementById('dropDown'), PlaceUp = document.getElementById('placeUp'), Place = document.getElementById('place'), PlaceDown = document.getElementById('placeDown');
    let SuckUp = document.getElementById('suckUp'), Suck = document.getElementById('suck'), SuckDown = document.getElementById('suckDown')
    DigUpButton.addEventListener('click', async () => {
        ID = idGenerator()
        var data = await MakeArrayAndSend(x, y, z, "digUp", direction)
        if (data.boolean != false & data.boolean != undefined) {
            AddInspectedBlocks(data,x,y,z)
        }
        UpdateInventory('', true)
    })
    DigButton.addEventListener('click', async () => {
        var data = await MakeArrayAndSend(x, y, z, "dig", direction)
        if (data.boolean != false & data.boolean != undefined) {
            AddInspectedBlocks(data,x,y,z)
        }
        UpdateInventory('', true)
    })
    DigDownButton.addEventListener('click', async () => {
        var data = await MakeArrayAndSend(x, y, z, "digDown", direction)
        if (data.boolean != false & data.boolean != undefined) {
            AddInspectedBlocks(data,x,y,z)
        }
        UpdateInventory('', true)
    })
    DropUp.addEventListener('click', async () => {
        let response = window.prompt("How many blocks to drop?","From 1 to 64")
        if (response != undefined) {
            if (isNaN(response) == true) {
                alert("АЛО, ТЫ СЧИТАЕШЬ ЧТО" + " " + response + " " + "ЭТО БЛЯТЬ ЧИСЛО?")
                broke = true
            } else {
                if (response > 64) {
                    response = 64
                } else if (response < 0) {
                    response = 0
                }
                var data = await sendData({message: "dropUp", count: response, name: TurtleName}, '/utility')
                if (data.boolean != false & data.boolean != undefined) {
                }
                UpdateInventory('', true)
            }
        }
    })
    Drop.addEventListener('click', async () => {
        let response = window.prompt("How many blocks to drop?","From 1 to 64")
        if (response != undefined) {
            if (isNaN(response) == true) {
                alert("АЛО, ТЫ СЧИТАЕШЬ ЧТО" + " " + response + " " + "ЭТО БЛЯТЬ ЧИСЛО?")
                broke = true
            } else {
                if (response > 64) {
                    response = 64
                } else if (response < 0) {
                    response = 0
                }
                var data = await sendData({message: "drop", count: response, name: TurtleName}, '/utility')
                if (data.boolean != false & data.boolean != undefined) {
                }
                UpdateInventory('', true)
            }
        }
    })
    DropDown.addEventListener('click', async () => {
        let response = window.prompt("How many blocks to drop?","From 1 to 64")
        if (response != undefined) {
            if (isNaN(response) == true) {
                alert("АЛО, ТЫ СЧИТАЕШЬ ЧТО" + " " + response + " " + "ЭТО БЛЯТЬ ЧИСЛО?")
                broke = true
            } else {
                if (response > 64) {
                    response = 64
                } else if (response < 0) {
                    response = 0
                }
                await sendData({message: "dropDown", count: response, name: TurtleName}, '/utility')
                UpdateInventory('', true)
            }
        }
    })
    PlaceUp.addEventListener('click', async () => {
        var data = await MakeArrayAndSend(x, y, z, "placeUp", direction)
        if (data.boolean != false & data.boolean != undefined) {
            AddInspectedBlocks(data,x,y,z)
        }
        UpdateInventory('', true)
    })
    Place.addEventListener('click', async () => {
        var data = await MakeArrayAndSend(x, y, z, "place", direction)
        if (data.boolean != false & data.boolean != undefined) {
            AddInspectedBlocks(data,x,y,z)
        }
        UpdateInventory('', true)
    })
    PlaceDown.addEventListener('click', async () => {
        var data = await MakeArrayAndSend(x, y, z, "placeDown", direction)
        if (data.boolean != false & data.boolean != undefined) {
            AddInspectedBlocks(data,x,y,z)
        }
        UpdateInventory('', true)
    })
    SuckUp.addEventListener('click', async () => {
        let response = window.prompt("How many blocks to suck?","From 1 to 64")
        if (response != undefined) {
            if (isNaN(response) == true) {
                alert("АЛО, ТЫ СЧИТАЕШЬ ЧТО" + " " + response + " " + "ЭТО БЛЯТЬ ЧИСЛО?")
                broke = true
            } else {
                if (response > 64) {
                    response = 64
                } else if (response < 0) {
                    response = 0
                }
                await sendData({message: "suckUp", count: response}, '/utility')
                UpdateInventory('', true)
            }
        }
    })
    Suck.addEventListener('click', async () => {
        let response = window.prompt("How many blocks to suck?","From 1 to 64")
        if (response != undefined) {
            if (isNaN(response) == true) {
                alert("АЛО, ТЫ СЧИТАЕШЬ ЧТО" + " " + response + " " + "ЭТО БЛЯТЬ ЧИСЛО?")
                broke = true
            } else {
                if (response > 64) {
                    response = 64
                } else if (response < 0) {
                    response = 0
                }
                await sendData({message: "suck", count: response}, '/utility')
                UpdateInventory('', true)
            }
        }
    })
    SuckDown.addEventListener('click', async () => {
        let response = window.prompt("How many blocks to suck?","From 1 to 64")
        if (response != undefined) {
            if (isNaN(response) == true) {
                alert("АЛО, ТЫ СЧИТАЕШЬ ЧТО" + " " + response + " " + "ЭТО ЧИСЛО?")
                broke = true
            } else {
                if (response > 64) {
                    response = 64
                } else if (response < 0) {
                    response = 0
                }
                await sendData({message: "suckDown", count: response}, '/utility')
                UpdateInventory('', true)
            }
        }
    })


    for (var i = 1; i < 17; ++i) {
        let button = document.getElementById('button' + i)
        button.addEventListener('click', async (kek) => {
            if (kek.ctrlKey == false) {
                await MakeArrayAndSend(x, y, z, "select" + kek.path[0].id.substring(6), direction)
                let id = parseInt(kek.path[0].id.substring(6), 10)
                let button = document.getElementById('button' + SelectedSlot)
                button.removeAttribute('onmouseover')
                button.removeAttribute('onmouseleave')
                button.removeAttribute('style')
                SelectedSlot = id
                SetSelectedButton(id)
            } else {
                let response = window.prompt("How many blocks to transfer?","From 1 to 64");
                if (response != undefined) {
                    let broke = false
                    if (isNaN(response) == true) {
                        alert("АЛО, ТЫ СЧИТАЕШЬ ЧТО" + " " + response + " " + "ЭТО БЛЯТЬ ЧИСЛО?")
                        broke = true
                    }
                    if (broke == false) {
                        if (response > 64) {
                            response = 64
                        } else if (response < 0) {
                            response = 0
                        }
                        if (response != 0) {
                            await sendData({message: "transfer", howMany: response, toSlot:kek.path[0].id.substring(6), name: TurtleName}, '/utility')
                            UpdateInventory(datochka.inventory, true)
                        }
                    }
                }
            }
        })
    }

    const gui = new GUI();
    const Movement = gui.addFolder('Movement');
        Movement.add(MovementButton1,'Forward');
        Movement.add(MovementButton2, 'Left');
        Movement.add(MovementButton3, 'Right');
        Movement.add(MovementButton4, 'clear');
        Movement.add(MovementButton5, 'Up');
        Movement.add(MovementButton6, "Down")
        Movement.add(MovementButton7, "Back")
    Movement.open();

    async function MakeArrayAndSend(x, y, z, where, direction) {
        if (TurtleName != false) {
            DataToSend = MakeAnArray(where, x, y, z, TurtleName, direction)
            data = await sendData(DataToSend, '/pepe')
        } else {
            alert("Return to the turtles page and select one")
        }
        return data
    };

    function SetPos(target, x, y, z) {
        target.position.set(x, y, z);
        controls.target.set(x, y, z);
    };

    function MakeAnArray(moveDir, x, y, z, toWho, dirr) {
        DataToSend = {
            x: x,
            y: y,
            z: z,
            dir: moveDir,
            who: toWho,
            direction: dirr
        }
        DataToSend = JSON.stringify(DataToSend)
        return DataToSend;
    };

    function addBlock(where, direction,x,y,z, color, blockname) {
        if (color == "Nocolor") {
            if (where == "up") {
                for (var i in scene.children) {
                    if (scene.children[i].position.x == x & scene.children[i].position.y == y + 1 & scene.children[i].position.z == z) {
                        scene.remove(scene.children[i])
                        break;
                    }
                }
            }
            if (where == "middle") {
                var coords = functions.forward(direction, x, z)
                let nx = coords[0], nz = coords[1];
                for (var i in scene.children) {
                    if (scene.children[i].position.x == nx & scene.children[i].position.y == y & scene.children[i].position.z == nz) {
                        scene.remove(scene.children[i])
                        break;
                    }
                }
            }
            if (where == "down") {
                for (var i in scene.children) {
                    if (scene.children[i].position.x == x & scene.children[i].position.y == y - 1 & scene.children[i].position.z == z) {
                        scene.remove(scene.children[i])
                        break;
                    }
                }
            }
            
        } else {
            let material;
            console.log(blockname)
            if (blockname == "minecraftwater") {
                material = new THREE.MeshBasicMaterial( {
                    color: parseInt(Number(color), 10),
                    opacity: 0.2,
                    transparent: true
                });
            } else {
                material = new THREE.MeshBasicMaterial( {
                    color: parseInt(Number(color), 10),
                    opacity: 0.8,
                    transparent: true
                });
            }
            let Block = new THREE.Mesh( geometry, material );
            scene.add(Block);
            var outlineMaterial;
            if (blockname == "minecraftwater") {
                outlineMaterial = new THREE.MeshBasicMaterial( { color: 0x000000, side: THREE.BackSide, transparent: true, opacity: 0.1 } );
            } else {
                outlineMaterial = new THREE.MeshBasicMaterial( { color: 0x000000, side: THREE.BackSide, transparent: true, opacity: 0.5 } );
            }
            let outlineMesh = new THREE.Mesh( geometry, outlineMaterial );
            outlineMesh.scale.multiplyScalar( 1.05 );
            Block.add( outlineMesh );
            let blockLabels = [];
            if (where == "up") {
                for (var i in scene.children) {
                    if (scene.children[i].position.x == x & scene.children[i].position.y == y + 1 & scene.children[i].position.z == z) {
                        scene.remove(scene.children[i])
                        break;
                    }
                }
                Block.on('mousedown', function (ev) {
                    if (ev.data.originalEvent.shiftKey == true) {
                        let blockDiv = document.createElement( 'div' );
                        blockDiv.className = 'label'
                        blockDiv.id = "aboba"
                        blockDiv.innerText = blockname;
                        blockLabel = new CSS2DObject(blockDiv);
                        blockLabel.position.set(x/1000, y/1000, z/1000)
                        Block.add(blockLabel)
                        blockLabels.push(Block)
                    }
                })
                Block.on('mouseout', function (ev) {
                    blockLabels.forEach((Block) => {
                        Block.remove(Block.children[1])
                    })
                })
                Block.position.set(x, y + 1, z);
            } else if (where == "middle") {
                var coords = functions.forward(direction, x, z)
                let nx = coords[0], nz = coords[1];
                for (var i in scene.children) {
                    if (scene.children[i].position.x == nx & scene.children[i].position.y == y & scene.children[i].position.z == nz) {
                        scene.remove(scene.children[i])
                        break;
                    }
                }
                Block.on('mousedown', function (ev) {
                    if (ev.data.originalEvent.shiftKey == true) {
                        let blockDiv = document.createElement( 'div' );
                        blockDiv.className = 'label'
                        blockDiv.id = "aboba"
                        blockDiv.innerText = blockname;
                        blockLabel = new CSS2DObject(blockDiv);
                        blockLabel.position.set(x/1000, y/1000, z/1000)
                        Block.add(blockLabel)
                        blockLabels.push(Block)
                    }
                })
                Block.on('mouseout', function (ev) {
                    blockLabels.forEach((Block) => {
                        Block.remove(Block.children[1])
                    })
                })
                Block.position.set(nx, y, nz)
            } else {
                for (var i in scene.children) {
                    if (scene.children[i].position.x == x & scene.children[i].position.y == y - 1 & scene.children[i].position.z == z) {
                        scene.remove(scene.children[i])
                        break;
                    }
                }
                Block.on('mousedown', function (ev) {
                    if (ev.data.originalEvent.shiftKey == true) {
                        let blockDiv = document.createElement( 'div' );
                        blockDiv.className = 'label'
                        blockDiv.id = "aboba"
                        blockDiv.innerText = blockname;
                        blockLabel = new CSS2DObject(blockDiv);
                        blockLabel.position.set(x/1000, y/1000, z/1000)
                        Block.add(blockLabel)
                        blockLabels.push(Block)
                    }
                })
                Block.on('mouseout', function (ev) {
                    blockLabels.forEach((Block) => {
                        Block.remove(Block.children[1])
                    })
                })
                Block.position.set(x, y - 1, z);
            }
        }
    };

    let blockLabel, blockLabels = [];
    function AddBlockPos(x, y, z, color, blockname) {
        let outlineMaterial, material;
        if (blockname == "minecraftwater") {
            material = new THREE.MeshBasicMaterial( {
                color: parseInt(Number(color), 10),
                opacity: 0.2,
                transparent: true
            });
        } else {
            material = new THREE.MeshBasicMaterial( {
                color: parseInt(Number(color), 10),
                opacity: 0.8,
                transparent: true
            });
        }
        let Block = new THREE.Mesh( geometry, material );
        scene.add(Block);
        Block.on('mousedown', function (ev) {
            if (ev.data.originalEvent.shiftKey == true) {
                let blockDiv = document.createElement( 'div' );
                blockDiv.className = 'label'
                blockDiv.id = "aboba"
                blockDiv.innerText = blockname;
                blockLabel = new CSS2DObject(blockDiv);
                blockLabel.position.set(x/1000, y/1000, z/1000)
                Block.add(blockLabel)
                blockLabels.push(Block)
            }
        })
        Block.on('mouseout', function (ev) {
            blockLabels.forEach((Block) => {
                Block.remove(Block.children[1])
            })
        })
        if (blockname == "minecraftwater") {
            outlineMaterial = new THREE.MeshBasicMaterial( { color: 0x000000, side: THREE.BackSide, transparent: true, opacity: 0.1 } );
        } else {
            outlineMaterial = new THREE.MeshBasicMaterial( { color: 0x000000, side: THREE.BackSide, transparent: true, opacity: 0.5 } );
        }
        let outlineMesh = new THREE.Mesh( geometry, outlineMaterial );
        outlineMesh.scale.multiplyScalar( 1.05 );
        Block.add( outlineMesh );
        Block.position.set(x, y, z)
    };

    function AddInspectedBlocks(data,x,y,z) {
        let colorUp = "Nocolor", colorMiddle = "Nocolor", colorDown = "Nocolor"
        console.log(data.colorUp, data.colorMiddle, data.colorDown)
        if (data.colorUp != undefined) {
            colorUp = data.colorUp
        }
        if (data.colorMiddle != undefined) {
            colorMiddle = data.colorMiddle
        }
        if (data.colorDown != undefined) {
            colorDown = data.colorDown
        }
        console.log(colorUp, " ", colorMiddle, " ", colorDown)
        if (data != null) {
            if (data.up) {
                addBlock("up", direction,x,y,z, colorUp, data.up)
            }
            if (data.down) {
                addBlock("down", direction,x,y,z, colorDown, data.down)
            }
            if (data.middle) {
                addBlock("middle", direction,x,y,z, colorMiddle, data.middle)
            }
            console.log(data.up, " | ", data.middle, " | ", data.down)     
        } else {
            console.log("message is empty")
        }
    };

    function idGenerator(){
        return Date.now().toString(36) + Math.random().toString(36).substring(2);
    }

    function SetSelectedButton(number) {
        let buttons = document.getElementById("button" + number)
        buttons.style.backgroundColor = 'rgb(156, 156, 156)';
        buttons.setAttribute("onmouseover", "over" + "(" + number + ")")
        buttons.setAttribute("onmouseleave", "leave" + "(" + number + ")")
    };

    async function UpdateInventory(inventory, NeedToRequestInventory) {
        if (NeedToRequestInventory == true) {
            inventory = await sendData({message: "inventory", name: TurtleName}, '/utility')
            inventory = inventory.inventory
        }
        for (i in inventory) {
            if (inventory[i] != undefined & i != 16) {
                let BlockName = inventory[i].name
                let BlockCount = inventory[i].count
                let b = parseInt(i, 10) + 1
                let buttons = document.getElementById("button" + b)
                buttons.innerHTML = BlockName + " x" + BlockCount 
            } else if (inventory[i] == undefined) {
                let b = parseInt(i, 10) + 1
                let button = document.getElementById('button' + b)
                if (button.innerHTML != undefined) {  
                    button.innerHTML = ""
                }
            }
        }
    };

    function clean() {
        if( removable_items.length > 0 ) {
          removable_items.forEach(function(v,i) {
             scene.remove(v);
          });
          removable_items = null;
          removable_items = [];
          Block = 1;
          SetPos(turtle, 0, 0, 0)
          x = 0, y = 0, z = 0
        }
    };

    function sleep(milliseconds) {
        const date = Date.now();
        let currentDate = null;
            do {
            currentDate = Date.now();
            } while (currentDate - date < milliseconds);
    };

    async function sendData(data, where, options = {}) {
        data = {message: data}

        const { timeout = 8000 } = options;
  
        const controller = new AbortController();
        const id = setTimeout(() => controller.abort(), timeout);

        try {
            return await fetch(where, {
              method: 'POST',
              body: JSON.stringify(data),
              headers: {
                'Content-Type': 'application/json'
              },
              signal: controller.signal
            })
            .then((response)=>response.json())
              .then((responseJson)=>{clearTimeout(id); return responseJson});
        } catch (error) {
            console.error("Error: " + error)
        }
        clearTimeout(id);
    };

    var WASD = document.getElementById("WASD");
    WASD.addEventListener('keydown', async function(event) {
        const key = event.key.toLowerCase();

        if (key == 'w') {
            let coords = functions.forward(direction, x, z);
                let xx = coords[0], zz = coords[1];
            var data = await MakeArrayAndSend(xx, y, zz, "forward", direction)
            if (data.boolean != false & data.boolean != undefined) {
                coords = functions.forward(direction, x, z);
                x = coords[0], z = coords[1];
                console.log(x, y, z)
                SetPos(turtle,x,y,z)
                AddInspectedBlocks(data,x,y,z)
            }
        } else if (key == 'a') {;
            let Tempdirection = functions.left(direction);
            var data = await MakeArrayAndSend(x, y, z, "left", Tempdirection)
            if (data.boolean != false & data.boolean != undefined) {
                direction = functions.left(direction);
                turtle.rotateZ(THREE.MathUtils.degToRad(-90));
                AddInspectedBlocks(data,x,y,z)
            }
        } else if (key == 'd') {;
            let Tempdirection = functions.left(direction);
            var data = await MakeArrayAndSend(x, y, z, "right", Tempdirection)
            if (data.boolean != false & data.boolean != undefined) {
                direction = functions.right(direction);
                turtle.rotateZ(THREE.MathUtils.degToRad(90));
                AddInspectedBlocks(data,x,y,z)
            }
        } else if (key == 's') {
            let coords = functions.back(direction, x, z);
                let xx = coords[0], zz = coords[1];
            var data = await MakeArrayAndSend(xx, y, zz, "back", direction)
            if (data.boolean != false & data.boolean != undefined) {
                coords = functions.back(direction, x, z);
                x = coords[0], z = coords[1];
                console.log(x, y, z)
                SetPos(turtle,x,y,z)
                AddInspectedBlocks(data,x,y,z)
        }
        } else if (key == 'q') {
            let Tosendy = y + 1
            var data = await MakeArrayAndSend(x, Tosendy, z, "up")
            if (data.boolean != false & data.boolean != undefined) {
                y = ++y
                SetPos(turtle,x,y,z)
                AddInspectedBlocks(data,x,y,z)
            }
        } else if (key == 'z') {
            let Tosendy = y - 1
            var data = await MakeArrayAndSend(x, Tosendy, z, "down")
            if (data.boolean != false & data.boolean != undefined) {
                y = --y
                SetPos(turtle,x,y,z)
                AddInspectedBlocks(data,x,y,z)
            }
        };

    });

    if (document.readyState == 'complete') {
        fetch('/UserConnected', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({message: "Someone connected to " + TurtleName})
        });
    };

    window.addEventListener('beforeunload', () => {
        fetch('/UserDisconnected', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({message: "Someone disconnected from " + TurtleName})
        })
    });
};

export default init();