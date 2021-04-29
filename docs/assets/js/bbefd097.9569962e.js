(window.webpackJsonp=window.webpackJsonp||[]).push([[22],{95:function(e,t,r){"use strict";r.r(t),r.d(t,"frontMatter",(function(){return o})),r.d(t,"metadata",(function(){return s})),r.d(t,"toc",(function(){return u})),r.d(t,"default",(function(){return b}));var n=r(3),a=r(7),i=(r(0),r(110)),o={sidebar_position:3},s={unversionedId:"tutorial-basics/MergeSort",id:"tutorial-basics/MergeSort",isDocsHomePage:!1,title:"Bubble Sort",description:"Bubble Sort is the simplest sorting algorithm that works by repeatedly swapping the adjacent elements if they are in wrong order.",source:"@site/docs/tutorial-basics/MergeSort.md",sourceDirName:"tutorial-basics",slug:"/tutorial-basics/MergeSort",permalink:"/docs/tutorial-basics/MergeSort",editUrl:"https://github.com/facebook/docusaurus/edit/master/website/docs/tutorial-basics/MergeSort.md",version:"current",sidebarPosition:3,frontMatter:{sidebar_position:3},sidebar:"tutorialSidebar",previous:{title:"Linear Search",permalink:"/docs/tutorial-basics/LinearSearch"},next:{title:"Selection Sort",permalink:"/docs/tutorial-basics/SelectionSort"}},u=[{value:"Code",id:"code",children:[]},{value:"Output",id:"output",children:[]}],c={toc:u};function b(e){var t=e.components,r=Object(a.a)(e,["components"]);return Object(i.b)("wrapper",Object(n.a)({},c,r,{components:t,mdxType:"MDXLayout"}),Object(i.b)("p",null,Object(i.b)("strong",{parentName:"p"},"Bubble Sort")," is the simplest sorting algorithm that works by repeatedly swapping the adjacent elements if they are in wrong order."),Object(i.b)("h2",{id:"code"},"Code"),Object(i.b)("pre",null,Object(i.b)("code",{parentName:"pre",className:"language-rust"},'fn bubble_sort(mut arr: [i32; 7]) -> [i32; 7] {\n    for i in 0..arr.len() {\n        for j in 0..arr.len() - i - 1 {\n            if arr[j] > arr[j + 1] {\n                let temp = arr[j];\n                arr[j] = arr[j + 1];\n                arr[j + 1] = temp;\n            }\n        }\n    }\n    arr\n}\nfn main() {\n    let mut arr = [64, 34, 25, 12, 22, 11, 9];\n    arr = bubble_sort(arr);\n    println!("Sorted array is {:?}", arr);\n} \n')),Object(i.b)("h2",{id:"output"},"Output"),Object(i.b)("pre",null,Object(i.b)("code",{parentName:"pre",className:"language-bash"},"    Finished dev [unoptimized + debuginfo] target(s) in 0.34s\n     Running `target/debug/algo`\nSorted array is [9, 11, 12, 22, 25, 34, 64] \n")))}b.isMDXComponent=!0}}]);