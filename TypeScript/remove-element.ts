function removeElement(nums: number[], val: number): number {
    let i = 0, j = nums.length - 1;
    while (i <= j) {
        nums[i] === val
            ? (nums[i] = nums[j], j--)
            : i++;
    }
    return i;
};
