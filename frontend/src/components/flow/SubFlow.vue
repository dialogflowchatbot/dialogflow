<script setup>
import { h, ref, onMounted, onUnmounted, nextTick, provide, readonly } from "vue";
import { useRoute, useRouter } from 'vue-router';
import CollectNode from './nodes/CollectNode.vue';
import ConditionNode from './nodes/ConditionNode.vue';
import DialogNode from './nodes/DialogNode.vue';
import KnowledgeBaseAnswerNode from './nodes/KnowledgeBaseAnswerNode.vue';
import EndNode from './nodes/EndNode.vue';
import GotoNode from './nodes/GotoNode.vue';
import ExternalHttpNode from './nodes/ExternalHttpNode.vue';
import SendEmailNode from './nodes/SendEmailNode.vue';
import LlmChatNode from './nodes/LlmChatNode.vue';
import { Graph } from '@antv/x6';
// https://x6.antv.vision/zh/docs/tutorial/advanced/react#%E6%B8%B2%E6%9F%93-vue-%E8%8A%82%E7%82%B9
import { register, getTeleport } from "@antv/x6-vue-shape";
import { atob, httpReq } from '../../assets/tools.js'
// import { ElNotification, ElMessage, ElMessageBox } from 'element-plus';
import { useI18n } from 'vue-i18n'
import EpDelete from '~icons/ep/delete'
import EpEdit from '~icons/ep/edit'
import EpFinished from '~icons/ep/finished'
import EpPlus from '~icons/ep/plus'
import EpPromotion from '~icons/ep/promotion'
import EpDArrowRight from '~icons/ep/d-arrow-right'
const { t, tm, rt } = useI18n();

const route = useRoute();
const router = useRouter();
const robotId = route.params.robotId;
// console.log(router.currentRoute.from)
const TeleportContainer = getTeleport();

const subFlows = ref([]);
const subflowNames = ref([])

function updateSubFlowNames() {
    // console.log(subFlows);
    const names = new Array();
    for (let i = 0; i < subFlows.value.length; i++) {
        // console.log(subFlows.value[i].id);
        names.push({ id: subFlows.value[i].id, name: subFlows.value[i].name });
    }
    // console.log(names);
    return names;
}

// provide('getSubFlowNames', {readonly(subflowNames), updateSubFlowNames})
provide('subFlowNamesFn', { subflowNames, updateSubFlowNames })
provide('robotId', { robotId })

register({
    shape: "CollectNode",
    width: 270,
    height: 120,
    component: CollectNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "ConditionNode",
    width: 270,
    height: 100,
    component: ConditionNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "DialogNode",
    width: 270,
    height: 100,
    component: DialogNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "KnowledgeBaseAnswerNode",
    width: 270,
    height: 150,
    component: KnowledgeBaseAnswerNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "GotoNode",
    width: 270,
    height: 100,
    component: GotoNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "ExternalHttpNode",
    width: 270,
    height: 100,
    component: ExternalHttpNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "SendEmailNode",
    width: 270,
    height: 100,
    component: SendEmailNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "EndNode",
    width: 270,
    height: 100,
    component: EndNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

register({
    shape: "LlmChatNode",
    width: 270,
    height: 120,
    component: LlmChatNode,
    ports: {
        groups: {
            absolute: {
                position: {
                    name: 'absolute',
                },
                attrs: {
                    circle: {
                        r: 5,
                        magnet: true,
                        stroke: 'black',
                        strokeWidth: 1,
                        fill: '#fff',
                        style: {
                            visibility: 'show',
                        },
                    },
                },
                label: {
                    position: 'left',
                }
            },
        },
    }
});

const nodes = [
    { name: tm('lang.flow.nodes')[0], type: 'DialogNode', desc: tm('lang.flow.nodesDesc')[0], cnt: 1 },
    { name: 'KnowledgeNode', type: 'KnowledgeBaseAnswerNode', desc: 'Knowledge base answer node', cnt: 1 },
    { name: 'LlmChatNode', type: 'LlmChatNode', desc: 'Llm chat node', cnt: 1 },
    { name: tm('lang.flow.nodes')[1], type: 'ConditionNode', desc: tm('lang.flow.nodesDesc')[1], cnt: 1 },
    { name: tm('lang.flow.nodes')[2], type: 'CollectNode', desc: tm('lang.flow.nodesDesc')[2], cnt: 1 },
    { name: 'ExternalHttpNode', type: 'ExternalHttpNode', desc: 'Request and send data to external HTTP API with variables', cnt: 1 },
    { name: 'SendEmailNode', type: 'SendEmailNode', desc: 'Sending an email an many recipients', cnt: 1 },
    { name: tm('lang.flow.nodes')[3], type: 'GotoNode', desc: tm('lang.flow.nodesDesc')[3], cnt: 1 },
    { name: 'EndNode', type: 'EndNode', desc: 'Ending node', cnt: 1 },
]
let selectedSubFlowIdx = -1;
// let offsetLeft = 0;
// let offsetTop = 0;
let graph = null;
let editedSubFlow = false;
const mainFlowId = route.params.id;
const mainFlowName = atob(route.params.name);
const isDemo = mainFlowId.indexOf('demo') > -1;

onMounted(async () => {
    // console.log("subflow onMounted");
    const canvas = document.getElementById('canvas');
    // offsetLeft = canvas.offsetLeft;
    // console.log('offsetLeft=' + offsetLeft);
    // offsetTop = canvas.offsetTop;
    // console.log('offsetTop=' + offsetTop);
    // console.log('offsetHeight=' + canvas.offsetHeight);
    graph = new Graph({
        container: canvas,
        width: canvas.offsetWidth - 10,
        height: canvas.offsetHeight,
        // height: 500,
        background: {
            color: "#F2F7FA",
        },
        autoResize: false,
        connecting: {
            allowBlank: false,
            allowLoop: false,
            allowNode: true,
            allowMulti: true,
            // http://x6.antv.antgroup.com/tutorial/basic/interacting#createedge
            createEdge() {
                return this.createEdge({
                    shape: 'edge',
                    attrs: {
                        line: {
                            stroke: '#8f8f8f',
                            strokeWidth: 1,
                            targetMarker: {
                                name: 'block',
                                width: 12,
                                height: 8,
                            },
                        },
                    },
                })
            },
        },
        // https://x6.antv.vision/zh/docs/api/graph/interaction#highlighting
        // 可以通过 graph.options.highlighting.magnetAvailable.attrs.xxx = xxx 动态修改样式。
        highlighting: {
            // 当链接桩可以被链接时，在链接桩外围渲染一个 2px 宽的红色矩形框
            magnetAvailable: {
                name: 'stroke',
                args: {
                    padding: 4,
                    attrs: {
                        'stroke-width': 2,
                        stroke: 'black',
                    }
                },
            },
        },
        panning: true,
    });
    graph.on('node:click', ({ e, x, y, node, view }) => {
        node.setTools([{
            name: 'button-remove',
            args: { x: 0, y: 0 },
        },]);
    });
    graph.on('node:mouseleave', ({ e, x, y, node, view }) => {
        if (node.hasTool("button-remove")) {
            node.removeTool("button-remove");
        }
    });
    graph.on('node:dblclick', ({ e, x, y, node, view }) => {
        node.setData({ currentTime: Date.now() });
        editedSubFlow = true;
    });
    graph.on("edge:click", ({ e, x, y, edge, view }) => {
        edge.setTools(['button-remove']);
    });

    const t = await httpReq('GET', 'subflow', { robotId: robotId, mainFlowId: mainFlowId, data: '' }, null, null);
    if (isDemo) {
        const d = { status: 200, data: t };
        cacheSubFlows(d);
    } else
        cacheSubFlows(t);
    nextTick(() => {
        showSubFlow(0);
    })
    // console.log("onMounted2");

});

onUnmounted(
    () => {
        if (graph != null)
            graph.dispose()
    }
);

function addHandleNode(x, y, item) {
    // console.log('addHandleNode' + x);
    const node = graph.addNode({
        shape: item.type,
        x: x,
        y: y,
        // tools: ["button-remove"],
    });
    item.cnt++;
    node.setData({ nodeType: item.type, nodeCnt: item.cnt });
    editedSubFlow = true;
}

function handleDragEnd(e, item) {
    const point = graph.pageToLocal(e.pageX, e.pageY);
    // addHandleNode(e.pageX - 150, e.pageY - 40, item);
    addHandleNode(point.x, point.y, item);
}

function dragoverDiv(ev) {
    ev.preventDefault();
}

function cacheSubFlows(t) {
    if (t && t.status == 200 && t.data) {
        subFlows.value = t.data;
        // showSubFlow(selectedSubFlowIdx);
    }
}

const dialogFormVisible = ref(false);
const flowName = ref('');
async function newSubFlow() {
    await saveSubFlow();
    const t = await httpReq('POST', 'subflow/new', { robotId: robotId, mainFlowId: mainFlowId, data: flowName.value }, null, null);
    if (t.status == 200) {
        const idx = subFlows.value.length;
        cacheSubFlows(t);
        nextTick(() => {
            showSubFlow(idx);
            flowName.value = '';
        })
    }
}

function removeSubFlow(index) {
    if (subFlows.value.length < 2) {
        ElMessage.error(t('lang.flow.needOne'));
    } else {
        ElMessageBox.confirm(
            t('lang.flow.delConfirm'),
            'Warning',
            {
                confirmButtonText: t('lang.common.del'),
                cancelButtonText: t('lang.common.cancel'),
                type: 'warning',
            }
        ).then(async () => {
            const r = await httpReq('DELETE', 'subflow', { robotId: robotId, mainFlowId: mainFlowId, data: selectedSubFlowIdx }, null, null);
            if (r.status == 200) {
                selectedSubFlowIdx = -1;
                subFlows.value.splice(index, 1);
                showSubFlow(0);
            }
            ElMessage({
                type: 'success',
                message: t('lang.common.deleted'),
            })
        }).catch(() => {
            // ElMessage({
            //     type: 'info',
            //     message: 'Delete canceled',
            // })
        })
    }
}

async function showSubFlow(idx) {
    if (idx == selectedSubFlowIdx)
        return;
    if (editedSubFlow) {
        // console.log('editedSubFlow')
        ElMessageBox.confirm(
            t('lang.flow.changeSaveTip'),
            'Warning',
            {
                confirmButtonText: t('lang.common.save'),
                cancelButtonText: t('lang.common.cancel'),
                type: 'warning',
            }
        ).then(async () => {
            await saveSubFlow();
            switchSubFlow(idx);
            editedSubFlow = false;
        }).catch(() => {
        })
    } else
        switchSubFlow(idx)
}

function switchSubFlow(idx) {
    const o = document.getElementById(subFlowId(selectedSubFlowIdx));
    if (o) {
        o.style.backgroundColor = 'white';
        o.style.color = 'black';
    }
    // console.log(idx);
    selectedSubFlowIdx = idx;
    // console.log(subFlowId(selectedSubFlowIdx));
    const n = document.getElementById(subFlowId(selectedSubFlowIdx));
    n.style.backgroundColor = 'rgb(245,246,249)';
    n.style.color = 'rgb(131,88,179)';
    // console.log(subFlows.value[selectedSubFlowIdx].canvas);
    // console.log(selectedSubFlowIdx);
    if (subFlows.value[selectedSubFlowIdx].canvas) {
        const canvas = JSON.parse(subFlows.value[selectedSubFlowIdx].canvas);
        const cells = canvas.cells;
        // subFlows.value[selectedSubFlowIdx].canvas = canvas;
        // console.log(subFlows.value[selectedSubFlowIdx].canvas);
        graph.fromJSON(cells);
    } else {
        graph.clearCells();
    }
}

async function saveSubFlow() {
    loading.value = true;
    saveLoading.value = true;
    const canvas = graph.toJSON();
    // console.log(canvas);
    const cells = canvas.cells;
    cells.forEach(function (item, index, arr) {
        if (item.shape != 'edge') {
            item.data.nodeId = item.id;
        }
    }, nodes);
    const source = subFlows.value[selectedSubFlowIdx];
    const cnts = [];
    for (let i = 0; i < nodes.length; i++)
        cnts.push(nodes[i].cnt);
    const data = {
        valid: false,
        id: source.id,
        name: source.name,
        canvas: JSON.stringify(canvas),
        // nodes: JSON.stringify(nodes),
        nodesStat: cnts,
    };
    const r = await httpReq('POST', 'subflow', { robotId: robotId, mainFlowId: mainFlowId, data: selectedSubFlowIdx }, null, data);
    // console.log(r);
    cacheSubFlows(r);
    ElNotification({
        title: t('lang.common.successTip'),
        message: h('b', { style: 'color: teal' }, t('lang.common.saved')),
        type: 'success',
    });
    saveLoading.value = false;
    loading.value = false;
    editedSubFlow = false;
}

function subFlowId(idx) {
    return 'subFlow' + idx.toString();
}

function goBack() {
    if (isDemo)
        router.go(-1);
    else
        router.push({ name: 'mainflows', params: { robotId: robotId } });
}

async function release() {
    loading.value = true;
    releaseLoading.value = true;
    if (!isDemo) {
        await saveSubFlow();
    }
    const r = await httpReq('GET', 'mainflow/release', { robotId: robotId, mainFlowId: mainFlowId, data: '' }, null, null);
    // console.log(r);
    if (r.status == 200) {
        ElNotification({
            title: t('lang.common.successTip'),
            message: h('b', { style: 'color: teal' }, t('lang.flow.subFlowReleased')),
            type: 'success',
        });
    } else {
        ElNotification({
            title: t('lang.common.errTip'),
            message: h('b', { style: 'color: teal' }, r.err.message),
            type: 'error',
        });
    }
    releaseLoading.value = false;
    loading.value = false;
}

// const formLabelWidth = '90px'
const loading = ref(false)
const saveLoading = ref(false);
const releaseLoading = ref(false);
const waitingResponse = ref(false)

const dryrunDisabled = ref(false);
const chatScrollbarRef = ref()
const dryrunChatRecords = ref();
const testingFormVisible = ref(false)
const userAsk = ref('')
const chatRecords = ref([])
let sessionId = '';
function newSessionId() {
    const d = Date.now().toString();
    return d + Math.random().toString(16);
}
function addChat(t, c, aT) {
    chatRecords.value.push({
        id: 'chat-' + Math.random().toString(16),
        text: t,
        cssClass: c,
        answerType: aT,
    });
}
async function dryrun() {
    if (chatRecords.value.length > 0 && !userAsk.value)
        return;
    waitingResponse.value = true;
    if (userAsk.value)
        addChat(userAsk.value, 'userText', 'TextPlan');
    if (!sessionId)
        sessionId = newSessionId();
    const req = {
        robotId: robotId,
        mainFlowId: mainFlowId,
        sessionId: sessionId,
        userInputResult: chatRecords.value.length == 0 || userAsk.value ? 'Successful' : 'Timeout',
        userInput: userAsk.value,
        importVariables: [],
        // userInputIntent: '',
    };
    const r = await httpReq('POST', 'flow/answer', null, null, req);
    // console.log(r);
    if (r.status == 200) {
        const data = r.data;
        const answers = data.answers;
        for (let i = 0; i < answers.length; i++)
            addChat(answers[i].text, 'responseText', answers[i].answerType);
        userAsk.value = '';
        if (data.nextAction == 'Terminate') {
            addChat(t('lang.flow.guideReset'), 'terminateText', 'TextPlain');
            dryrunDisabled.value = true;
        }
        nextTick(() => {
            // console.log(dryrunChatRecords.value.clientHeight);
            chatScrollbarRef.value.setScrollTop(dryrunChatRecords.value.clientHeight);
        })
    } else {
        ElNotification({
            title: t('lang.common.errTip'),
            message: h('b', { style: 'color: teal' }, r.err.message),
            type: 'error',
        });
    }
    waitingResponse.value = false;
    dryrunInput.value.focus();
}
async function dryrunClear() {
    chatRecords.value.splice(0, chatRecords.value.length);
    userAsk.value = '';
    sessionId = '';
    dryrunDisabled.value = false;
    await dryrun();
}

// const isEnLanguage = navigator.language ? navigator.language.split('-')[0] == 'en' : false
// const nodesBtnWidth = isEnLanguage ? ref('100px') : ref('50px')

const dryrunInput = ref()
const popupRundryWindow = async () => {
    testingFormVisible.value = true;
    await dryrun();
}
</script>
<style scoped>
.el-container,
.el-header,
.el-main,
.el-footer {
    padding: 0;
}

.el-main {
    position: relative !important;
}

#canvas {
    min-height: 85vh;
}

.node-btn {
    cursor: pointer;
    border: 1px solid #eee;
    padding: 10px;
    margin-bottom: 6px;
    font-size: 9pt;
    /* width: v-bind(nodesBtnWidth); */
    width: 100px;
    background-color: white;
}

.DialogNode {
    border-left: 5px solid rgb(255, 196, 0);
}

.KnowledgeBaseAnswerNode {
    border-left: 5px solid #EFB7BA;
}

.ConditionNode {
    border-left: 5px solid rgb(145, 113, 227);
}

.CollectNode {
    border-left: 5px solid rgb(90, 213, 235);
}

.GotoNode {
    border-left: 5px solid rgb(67, 211, 153);
}

.ExternalHttpNode {
    border-left: 5px solid rgb(1, 165, 188);
}

.SendEmailNode {
    border-left: 5px solid rgb(255, 101, 85);
}

.EndNode {
    border-left: 5px solid rgb(34, 25, 106);
}

.LlmChatNode {
    border-left: 5px solid #6a2c70;
}

.nodesBox {
    display: flex;
    flex-direction: column;
    position: absolute;
    top: 20px;
    left: 20px;
    z-index: 100;
    width: 100px;
}

.subFlowBtn {
    padding: 5px;
    border-bottom: gray solid 1px;
    cursor: pointer;
    font-size: 13px;
}

.userText {
    text-align: right;
    margin-bottom: 16px;
}

.userText span {
    padding: 4px;
    border: #91b6ff 1px solid;
    border-radius: 6px;
    background-color: #f1f6ff;
}

.responseText {
    text-align: left;
    margin-bottom: 16px;
}

.responseText span {
    padding: 4px;
    border: #8bda1d 1px solid;
    border-radius: 6px;
    background-color: #efffd8;
    white-space: pre-wrap;
    display: inline-block;
}

.terminateText {
    text-align: center;
    margin-bottom: 16px;
}

.terminateText span {
    padding: 4px;
    border: #d3d3d3 1px solid;
    border-radius: 6px;
    background-color: #ebebeb;
    white-space: pre-wrap;
    display: inline-block;
}

.newSubFlowBtn {
    color: white;
    padding-left: 5px;
    padding-top: 5px;
    padding-bottom: 5px;
    background: linear-gradient(45deg, #e68dbf, pink);
    cursor: pointer;
    font-size: 16px;
}
</style>
<template>
    <div>
        <!-- <div id="modal-container"></div> -->
        <el-container style="min-height: 100vh;max-height: 100vh;">
            <el-header height="40px">
                <el-page-header :title="t('lang.common.back')" @back="goBack">
                    <template #content>
                        <span class="text-large font-600 mr-3">{{ mainFlowName }}</span>
                    </template>
                    <template #extra>
                        <div class="flex items-center">
                            <el-text v-show="isDemo">
                                {{ $tm('lang.flow.steps')[0] }}
                                <el-icon :size="20">
                                    <EpDArrowRight />
                                </el-icon>
                            </el-text>
                            <el-button type="primary" class="ml-2" @click="saveSubFlow" :loading="saveLoading"
                                size="large" v-show="!isDemo">
                                <el-icon :size="20">
                                    <EpEdit />
                                </el-icon>{{ $t('lang.flow.save') }}
                            </el-button>
                            <el-button type="success" @click="release" :loading="releaseLoading" size="large">
                                <el-icon :size="20">
                                    <EpFinished />
                                </el-icon>{{ $t('lang.flow.pub') }}
                            </el-button>
                            <!-- <div class="testBtn" @click="dryrun(); testingFormVisible = true">
                                <el-icon>
                                    <Promotion />
                                </el-icon>
                                测试流程
                            </div> -->
                            <el-text v-show="isDemo">
                                {{ $tm('lang.flow.steps')[1] }}
                                <el-icon>
                                    <EpDArrowRight />
                                </el-icon>
                            </el-text>
                            <el-button color="#626aef" class="ml-2" @click="popupRundryWindow" size="large">
                                <el-icon :size="20">
                                    <EpPromotion />
                                </el-icon>
                                {{ $t('lang.flow.test') }}
                            </el-button>
                        </div>
                    </template>
                </el-page-header>
            </el-header>
            <el-container>
                <el-aside width="150px">
                    <div class="newSubFlowBtn" @click="dialogFormVisible = true">
                        <el-icon size="16px">
                            <EpPlus />
                        </el-icon>
                        {{ $t('lang.flow.addSubFlow') }}
                    </div>
                    <div v-for="(item, index) in subFlows" :id="subFlowId(index)" :key="item.label"
                        @click="showSubFlow(index)" class="subFlowBtn">
                        {{ item.name }}
                        <span @click="removeSubFlow(index)">
                            <el-icon>
                                <EpDelete />
                            </el-icon>
                        </span>
                    </div>
                </el-aside>
                <el-main v-loading="loading">
                    <div class="nodesBox">
                        <!-- <div style="font-size: 9pt;">拖动下方节点</div> -->
                        <div v-for="item in nodes" :key="item.type" class="node-btn" :class="item.type" draggable="true"
                            @dragend="handleDragEnd($event, item)">
                            <el-tooltip class="box-item" effect="dark" :content="item.desc" placement="right-start">
                                <span> {{ item.name }}</span>
                            </el-tooltip>
                        </div>
                    </div>
                    <div id="canvas" @dragover="dragoverDiv" style="border: 1px #000 solid;"></div>
                    <TeleportContainer />
                </el-main>
            </el-container>
        </el-container>
        <el-dialog v-model="dialogFormVisible" :title="$t('lang.flow.addSubFlow')">
            <el-form :model="form">
                <el-form-item :label="t('lang.flow.form.name')" label-width="110px">
                    <el-input v-model="flowName" autocomplete="off" />
                </el-form-item>
            </el-form>
            <template #footer>
                <span class="dialog-footer">
                    <el-button type="primary" @click="dialogFormVisible = false; newSubFlow();">
                        {{ $t('lang.common.add') }}
                    </el-button>
                    <el-button @click="dialogFormVisible = false">{{ $t('lang.common.cancel') }}</el-button>
                </span>
            </template>
        </el-dialog>
        <el-drawer v-model="testingFormVisible" direction="rtl">
            <template #header>
                <b>{{ $t('lang.flow.test') }}</b>
            </template>
            <template #default>
                <el-scrollbar ref="chatScrollbarRef" height="100%" always>
                    <div ref="dryrunChatRecords">
                        <div v-for="item in chatRecords" :key="item.id" :class="item.cssClass">
                            <!-- <span v-html="item.text"></span> -->
                            <el-text v-if="item.answerType == 'TextPlain'">{{ item.text }}</el-text>
                            <el-text v-else v-html="item.text"></el-text>
                        </div>
                    </div>
                </el-scrollbar>
            </template>
            <template #footer>
                <div style="flex: auto">
                    <el-input ref="dryrunInput" :disabled="dryrunDisabled" v-model="userAsk" placeholder=""
                        style="width: 200px" @keypress="(e) => { if (e.keyCode == 13) dryrun(); }" />
                    <el-button-group>
                        <el-button type="primary" :disabled="dryrunDisabled" @click="dryrun"
                            :loading="waitingResponse">{{ $t('lang.flow.send')
                            }}</el-button>
                        <el-button @click="dryrunClear">{{ $t('lang.flow.reset') }}</el-button>
                    </el-button-group>
                </div>
            </template>
        </el-drawer>
    </div>
</template>
