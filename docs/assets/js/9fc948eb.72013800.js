(window.webpackJsonp=window.webpackJsonp||[]).push([[16],{89:function(e,t,n){"use strict";n.r(t),n.d(t,"frontMatter",(function(){return o})),n.d(t,"metadata",(function(){return s})),n.d(t,"toc",(function(){return u})),n.d(t,"default",(function(){return d}));var r=n(3),i=n(7),a=(n(0),n(110)),o={sidebar_position:4},s={unversionedId:"tutorial-basics/SelectionSort",id:"tutorial-basics/SelectionSort",isDocsHomePage:!1,title:"Selection Sort",description:"The selection sort algorithm sorts an array by repeatedly finding the minimum element (considering ascending order) from unsorted part and putting it at the beginning. The algorithm maintains two subarrays in a given array.",source:"@site/docs/tutorial-basics/SelectionSort.md",sourceDirName:"tutorial-basics",slug:"/tutorial-basics/SelectionSort",permalink:"/docs/tutorial-basics/SelectionSort",editUrl:"https://github.com/facebook/docusaurus/edit/master/website/docs/tutorial-basics/SelectionSort.md",version:"current",sidebarPosition:4,frontMatter:{sidebar_position:4},sidebar:"tutorialSidebar",previous:{title:"Bubble Sort",permalink:"/docs/tutorial-basics/MergeSort"}},u=[{value:"Code",id:"code",children:[]},{value:"Output",id:"output",children:[]}],c={toc:u};function d(e){var t=e.components,n=Object(i.a)(e,["components"]);return Object(a.b)("wrapper",Object(r.a)({},c,n,{components:t,mdxType:"MDXLayout"}),Object(a.b)("p",null,"The selection sort algorithm sorts an array by repeatedly finding the minimum element (considering ascending order) from unsorted part and putting it at the beginning. The algorithm maintains two subarrays in a given array."),Object(a.b)("p",null,"1) The subarray which is already sorted."),Object(a.b)("p",null,"2) Remaining subarray which is unsorted."),Object(a.b)("p",null,"In every iteration of selection sort, the minimum element (considering ascending order) from the unsorted subarray is picked and moved to the sorted subarray."),Object(a.b)("h2",{id:"code"},"Code"),Object(a.b)("pre",null,Object(a.b)("code",{parentName:"pre",className:"language-rust"},'fn selection_sort(mut arr: [i32; 7]) -> [i32; 7] {\n    let mut min;\n    for i in 0..arr.len() - 1 {\n        min = i;\n        for j in i + 1..arr.len() {\n            // Finding the minimum element in the array\n            if arr[j] < arr[min] {\n                min = j;\n            }\n        }\n        // Swap the found minimum element with the first element\n        let temp = arr[i];\n        arr[i] = arr[min];\n        arr[min] = temp;\n    }\n    arr\n}\nfn main() {\n    let mut arr = [64, 34, 25, 12, 22, 11, 9];\n    arr = selection_sort(arr);\n    println!("Sorted array is {:?}", arr);\n}\n')),Object(a.b)("h2",{id:"output"},"Output"),Object(a.b)("pre",null,Object(a.b)("code",{parentName:"pre",className:"language-bash"},"    Finished dev [unoptimized + debuginfo] target(s) in 0.53s\n     Running `target/debug/algo`\nSorted array is [9, 11, 12, 22, 25, 34, 64]\n")))}d.isMDXComponent=!0}}]);