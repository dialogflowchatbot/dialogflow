<template>
  <el-tiptap v-model:content="content" :extensions="extensions" @blur="doCallback" />
</template>
  
<script setup>
import { ref, toRef } from 'vue';
import {
  // necessary extensions
  Doc,
  Text,
  Paragraph,
  //________________________
  Heading,
  FontSize,
  Bold,
  Underline,
  Italic,
  Strike,
  BulletList,
  OrderedList,
  Link,
  Table,
  Image,
  TaskList,
  TextAlign,
  FormatClear,
} from 'element-tiptap-vue3-fixed';

// editor extensions
// they will be added to menubar and bubble menu by the order you declare.
const extensions = [
  Doc,
  Text,
  Paragraph,
  Heading.configure({ level: 5 }),
  FontSize,
  Bold.configure({ bubble: true }), // render command-button in bubble menu.
  Underline.configure({ bubble: true, menubar: false }), // render command-button in bubble menu but not in menubar.
  Italic,
  Strike,
  BulletList,
  OrderedList,
  Link,
  Table,
  Image,
  TaskList,
  TextAlign,
  FormatClear,
];

// editor's content
const content = ref(`
    <h1>Heading</h1>
    <p>This Editor is awesome!</p>
  `);
const props = defineProps({
  editorText: String
})
// console.log("props.editorText="+props.editorText)
// const content = toRef(props, 'editorText')
// console.log("content="+content)

content.value = props.editorText

const emit = defineEmits(['updatedEditorText'])

const doCallback = () => {
  console.log('called callback')
}

</script>