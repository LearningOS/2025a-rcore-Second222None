# lab2

MemorySet用来表示一个Task的内存信息，内部包含页表和MemArea。mmap所做的事情包括：
- 找个一个物理页，映射到页表中
- 将虚拟地址转为MemArea插入到areas字段中

可调用MemorySet中的`insert_framed_area()`。`unmap()`所做的事情与mmap相反：
- 找对应的物理页，从页表中移除映射
- 将虚拟地址对应的MemArea从areas字段中移除